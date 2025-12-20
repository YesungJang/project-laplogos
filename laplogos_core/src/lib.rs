use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};

mod renderer;
mod store;
pub mod materialize;
pub mod diff;

pub use renderer::render_to_string;
pub use store::LaplogosState;
use store::{apply_action, normalize_action, validate_action, LaplogosConfig, ValidatedAction};
use crate::materialize::{materialize_one_root, LaplogosError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaplogosPage {
    pub v: u8,
    pub root: LaplogosNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaplogosNode {
    #[serde(rename = "i")]
    pub i: String,
    #[serde(rename = "t")]
    pub t: String,
    #[serde(rename = "p", default)]
    pub p: HashMap<String, Value>,
    #[serde(rename = "c", default)]
    pub c: Option<Vec<LaplogosNode>>,
    #[serde(rename = "if", default)]
    pub if_: Option<Value>,
    #[serde(rename = "for", default)]
    pub for_: Option<Value>,
    #[serde(rename = "on", default)]
    pub on: Option<HashMap<String, Vec<Value>>>,
}

#[wasm_bindgen]
pub struct LaplogosEngine {
    inner: Rc<RefCell<LaplogosEngineInner>>,
}

struct LaplogosEngineInner {
    state: LaplogosState,
    page: LaplogosPage,
    logs: Vec<String>,
    last_materialized: Option<LaplogosNode>,
    config: Option<LaplogosConfig>,
}

impl Clone for LaplogosEngine {
    fn clone(&self) -> Self {
        LaplogosEngine {
            inner: self.inner.clone(),
        }
    }
}

#[wasm_bindgen]
impl LaplogosEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(initial_json: &str) -> Result<LaplogosEngine, JsValue> {
        let page: LaplogosPage = serde_json::from_str(initial_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid JSON: {e}")))?;

        Ok(LaplogosEngine {
            inner: Rc::new(RefCell::new(LaplogosEngineInner {
                state: LaplogosState::default(),
                page,
                logs: Vec::new(),
                last_materialized: None,
                config: None,
            })),
        })
    }

    pub fn get_state_json(&self) -> String {
        let inner = self.inner.borrow();
        serde_json::to_string(&inner.state.data).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn init(&self, config_json: &str) -> Result<(), JsValue> {
        let mut config: LaplogosConfig = serde_json::from_str(config_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid config JSON: {e}")))?;
        config.allowed_actions = config
            .allowed_actions
            .into_iter()
            .map(|a| a.to_lowercase())
            .collect();
        config.allowed_hosts = config
            .allowed_hosts
            .into_iter()
            .map(|h| h.to_lowercase())
            .collect();
        self.inner.borrow_mut().config = Some(config);
        Ok(())
    }

    pub fn dispatch(&self, action_json: &str) -> Result<(), JsValue> {
        self.dispatch_with_event(action_json, None)
    }

    pub fn rerender(&self, container_id: &str) -> Result<(), JsValue> {
        renderer::update_dom(self, container_id)
    }
}

impl LaplogosEngine {
    pub(crate) fn log_warn(&self, message: &str) {
        web_sys::console::warn_1(&JsValue::from_str(message));
        self.inner.borrow_mut().logs.push(message.to_string());
    }

    fn with_page<R>(&self, f: impl FnOnce(&LaplogosPage) -> R) -> R {
        let inner = self.inner.borrow();
        f(&inner.page)
    }

    pub(crate) fn dispatch_with_event(
        &self,
        action_json: &str,
        event_value: Option<String>,
    ) -> Result<(), JsValue> {
        let raw: Value = match serde_json::from_str(action_json) {
            Ok(value) => value,
            Err(e) => {
                self.log_warn(&format!("Invalid action JSON: {e}"));
                return Ok(());
            }
        };

        let action = match normalize_action(&raw) {
            Ok(action) => action,
            Err(err) => {
                self.log_warn(&err);
                return Ok(());
            }
        };

        if !self.is_action_allowed(&action.t) {
            self.log_warn(&format!("Action not allowed: {}", action.t));
            return Ok(());
        }

        let validated = match validate_action(action) {
            Ok(validated) => validated,
            Err(err) => {
                self.log_warn(&err);
                return Ok(());
            }
        };

        match validated {
            ValidatedAction::Fetch {
                url,
                on_success,
                on_fail,
            } => {
                self.spawn_fetch(url, on_success, on_fail);
                Ok(())
            }
            other => {
                let changed = {
                    let mut inner = self.inner.borrow_mut();
                    apply_action(&mut inner.state, other, event_value)
                };

                let changed = match changed {
                    Ok(changed) => changed,
                    Err(err) => {
                        self.log_warn(&err);
                        return Ok(());
                    }
                };

                if !changed {
                    return Ok(());
                }

                self.update_view("aix-root")
            }
        }
    }

    pub(crate) fn page_root(&self) -> LaplogosNode {
        self.with_page(|page| page.root.clone())
    }

    fn update_view(&self, container_id: &str) -> Result<(), JsValue> {
        let (materialized, warnings) = self.materialize_current()?;
        for warn in warnings {
            self.log_warn(&warn);
        }

        let previous = self.inner.borrow().last_materialized.clone();
        if let Some(old) = previous {
            let patches = diff::diff(&old, &materialized);
            if let Err(err) = renderer::apply_patches(self, container_id, &patches, &materialized) {
                self.log_warn(&format!("Patch apply failed: {:?}", err));
                renderer::update_dom_with_root(container_id, &materialized, Some(self))?;
            }
        } else {
            renderer::update_dom_with_root(container_id, &materialized, Some(self))?;
        }

        self.inner.borrow_mut().last_materialized = Some(materialized);
        Ok(())
    }

    fn materialize_current(&self) -> Result<(LaplogosNode, Vec<String>), JsValue> {
        let mut warnings = Vec::new();
        let materialized = {
            let inner = self.inner.borrow();
            materialize_one_root(&inner.page.root, &inner.state, &mut warnings)
        };
        match materialized {
            Ok(node) => Ok((node, warnings)),
            Err(err) => {
                self.log_error(&err);
                Err(JsValue::from_str(&err.message))
            }
        }
    }

    fn log_error(&self, err: &LaplogosError) {
        web_sys::console::error_1(&JsValue::from_str(&err.message));
        self.inner.borrow_mut().logs.push(err.message.clone());
    }

    fn spawn_fetch(&self, url: String, on_success: Vec<Value>, on_fail: Vec<Value>) {
        let engine = self.clone();
        spawn_local(async move {
            let parsed = match web_sys::Url::new(&url) {
                Ok(url) => url,
                Err(_) => {
                    engine.log_warn("fetch.url is invalid");
                    return;
                }
            };

            let host = parsed.host().to_lowercase();
            if host.is_empty() || !engine.is_host_allowed(&host) {
                engine.log_warn("fetch host is not allowed");
                return;
            }

            let window = match web_sys::window() {
                Some(window) => window,
                None => {
                    engine.log_warn("No window for fetch");
                    return;
                }
            };

            let resp_value = JsFuture::from(window.fetch_with_str(&url)).await;
            let ok = match resp_value {
                Ok(val) => val
                    .dyn_into::<web_sys::Response>()
                    .map(|resp| resp.ok())
                    .unwrap_or(false),
                Err(_) => false,
            };

            if ok {
                engine.dispatch_actions(on_success);
            } else {
                engine.dispatch_actions(on_fail);
            }
        });
    }

    fn dispatch_actions(&self, actions: Vec<Value>) {
        for action in actions {
            match serde_json::to_string(&action) {
                Ok(text) => {
                    let _ = self.dispatch_with_event(&text, None);
                }
                Err(_) => self.log_warn("Failed to serialize action"),
            }
        }
    }

    fn is_action_allowed(&self, action: &str) -> bool {
        let inner = self.inner.borrow();
        match inner.config.as_ref() {
            Some(config) => config.allows_action(action),
            None => true,
        }
    }

    pub(crate) fn is_host_allowed(&self, host: &str) -> bool {
        let inner = self.inner.borrow();
        match inner.config.as_ref() {
            Some(config) => config.allows_host(host),
            None => false,
        }
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn render_laplogos(json_str: &str, container_id: &str) -> Result<(), JsValue> {
    let page: LaplogosPage =
        serde_json::from_str(json_str).map_err(|e| JsValue::from_str(&format!("Invalid JSON: {e}")))?;

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("No document"))?;
    let container = document
        .get_element_by_id(container_id)
        .ok_or_else(|| JsValue::from_str("Container not found"))?;

    container.set_inner_html("");

    let root_el = renderer::render_node(&document, &page.root, None)?;
    container.append_child(&root_el)?;

    Ok(())
}
