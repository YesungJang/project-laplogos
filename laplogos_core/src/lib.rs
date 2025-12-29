use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use js_sys::{Date, Function, JSON};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};

mod renderer;
mod store;
pub mod materialize;
pub mod diff;
pub mod normalize;

pub use renderer::render_to_string;
pub use normalize::normalize_page;
pub use store::LaplogosState;
use store::{apply_action, normalize_action, validate_action, LaplogosConfig, ValidatedAction};
use crate::materialize::{materialize_one_root, LaplogosError};

const SCHEMA_VERSION_DEFAULT: u8 = 1;
const MAX_ACTION_LOG_COUNT: usize = 200;
const MAX_ACTION_LOG_BYTES: usize = 64 * 1024;
const MAX_ERROR_LOG_COUNT: usize = 200;
const MAX_ERROR_LOG_BYTES: usize = 64 * 1024;
const MAX_PATCHES_PER_DISPATCH: usize = 2000;
const MAX_DISPATCH_PER_SEC: usize = 30;
const FETCH_TIMEOUT_MS: i32 = 10_000;
const FETCH_MAX_JSON_BYTES: usize = 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaplogosPage {
    #[serde(default = "default_schema_version")]
    pub v: u8,
    pub root: LaplogosNode,
    #[serde(rename = "defs", default, skip_serializing_if = "Option::is_none")]
    pub defs: Option<HashMap<String, LaplogosNode>>,
    #[serde(rename = "styles", default, skip_serializing_if = "Option::is_none")]
    pub styles: Option<HashMap<String, HashMap<String, String>>>,
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
    #[serde(rename = "r", default)]
    pub r: Option<String>,
    #[serde(rename = "st", default)]
    pub st: Option<String>,
    #[serde(rename = "o", default)]
    pub o: Option<HashMap<String, Value>>,
}

#[wasm_bindgen]
pub struct LaplogosEngine {
    inner: Rc<RefCell<LaplogosEngineInner>>,
}

struct LaplogosEngineInner {
    state: LaplogosState,
    input_page: Option<Value>,
    normalized_page: LaplogosPage,
    action_log: LogBuffer,
    error_log: LogBuffer,
    last_materialized: Option<LaplogosNode>,
    config: Option<LaplogosConfig>,
    on_nav: Option<Function>,
    metrics: EngineMetrics,
    dispatch_times: VecDeque<f64>,
}

#[derive(Debug, Default)]
struct LogBuffer {
    entries: VecDeque<String>,
    total_bytes: usize,
    max_entries: usize,
    max_bytes: usize,
}

impl LogBuffer {
    fn new(max_entries: usize, max_bytes: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            total_bytes: 0,
            max_entries,
            max_bytes,
        }
    }

    fn push(&mut self, entry: String) {
        self.total_bytes += entry.len();
        self.entries.push_back(entry);
        while self.entries.len() > self.max_entries || self.total_bytes > self.max_bytes {
            if let Some(removed) = self.entries.pop_front() {
                self.total_bytes = self.total_bytes.saturating_sub(removed.len());
            } else {
                break;
            }
        }
    }

    fn to_json_string(&self) -> String {
        let entries: Vec<&String> = self.entries.iter().collect();
        serde_json::to_string(&entries).unwrap_or_else(|_| "[]".to_string())
    }
}

#[derive(Debug, Default, Serialize)]
struct EngineMetrics {
    normalize_ms: Option<f64>,
    materialize_ms: Option<f64>,
    diff_ms: Option<f64>,
    patch_ms: Option<f64>,
    hydrate_ms: Option<f64>,
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
        let raw: Value = serde_json::from_str(initial_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid JSON: {e}")))?;
        let (schema_v, v_missing) = extract_schema_version(&raw)
            .map_err(|e| JsValue::from_str(&e.message))?;
        if let Err(err) = ensure_supported_schema_version(schema_v) {
            self.log_error(&err);
            return Err(JsValue::from_str(&err.message));
        }

        let normalize_start = Date::now();
        let raw_clone = raw.clone();
        let page = normalize_page(raw).map_err(|e| JsValue::from_str(&e.message))?;
        let normalize_ms = Date::now() - normalize_start;

        let engine = LaplogosEngine {
            inner: Rc::new(RefCell::new(LaplogosEngineInner {
                state: LaplogosState::default(),
                input_page: Some(raw_clone),
                normalized_page: page,
                action_log: LogBuffer::new(MAX_ACTION_LOG_COUNT, MAX_ACTION_LOG_BYTES),
                error_log: LogBuffer::new(MAX_ERROR_LOG_COUNT, MAX_ERROR_LOG_BYTES),
                last_materialized: None,
                config: Some(LaplogosConfig::default()),
                on_nav: None,
                metrics: EngineMetrics {
                    normalize_ms: Some(normalize_ms),
                    ..EngineMetrics::default()
                },
                dispatch_times: VecDeque::new(),
            })),
        };

        if v_missing {
            engine.log_warn("schema v is missing; defaulting to v1");
        }

        Ok(engine)
    }

    pub fn get_state_json(&self) -> String {
        let inner = self.inner.borrow();
        serde_json::to_string(&inner.state.data).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn get_action_log_json(&self) -> String {
        let inner = self.inner.borrow();
        inner.action_log.to_json_string()
    }

    pub fn get_error_log_json(&self) -> String {
        let inner = self.inner.borrow();
        inner.error_log.to_json_string()
    }

    pub fn get_page_json(&self) -> String {
        let inner = self.inner.borrow();
        serde_json::to_string(&inner.normalized_page).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn get_input_page_json(&self) -> String {
        let inner = self.inner.borrow();
        match inner.input_page.as_ref() {
            Some(value) => serde_json::to_string(value).unwrap_or_else(|_| "{}".to_string()),
            None => "{}".to_string(),
        }
    }

    pub fn get_metrics_json(&self) -> String {
        let inner = self.inner.borrow();
        serde_json::to_string(&inner.metrics).unwrap_or_else(|_| "{}".to_string())
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

    pub fn set_on_nav(&self, callback: Function) {
        self.inner.borrow_mut().on_nav = Some(callback);
    }

    pub fn dispatch(&self, action_json: &str) -> Result<(), JsValue> {
        self.dispatch_with_event(action_json, None)
    }

    pub fn set_page_json(&self, page_json: &str) -> Result<(), JsValue> {
        let raw: Value = serde_json::from_str(page_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid JSON: {e}")))?;
        let (schema_v, v_missing) = extract_schema_version(&raw)
            .map_err(|e| JsValue::from_str(&e.message))?;
        ensure_supported_schema_version(schema_v)
            .map_err(|e| JsValue::from_str(&e.message))?;

        let normalize_start = Date::now();
        let raw_clone = raw.clone();
        let page = normalize_page(raw).map_err(|e| JsValue::from_str(&e.message))?;
        let normalize_ms = Date::now() - normalize_start;

        let mut inner = self.inner.borrow_mut();
        inner.input_page = Some(raw_clone);
        inner.normalized_page = page;
        inner.last_materialized = None;
        inner.metrics.normalize_ms = Some(normalize_ms);

        if v_missing {
            drop(inner);
            self.log_warn("schema v is missing; defaulting to v1");
        }

        Ok(())
    }

    pub fn set_state_json(&self, state_json: &str) -> Result<(), JsValue> {
        let raw: Value = serde_json::from_str(state_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid JSON: {e}")))?;
        let state = parse_state(raw)?;
        let mut inner = self.inner.borrow_mut();
        inner.state = state;
        inner.last_materialized = None;
        Ok(())
    }

    pub fn rerender(&self, container_id: &str) -> Result<(), JsValue> {
        self.update_view(container_id)
    }

    pub fn hydrate(&self, container_id: &str) -> Result<(), JsValue> {
        let hydrate_start = Date::now();
        let (materialized, warnings) = self.materialize_current()?;
        for warn in warnings {
            self.log_warn(&warn);
        }

        match renderer::bind_events(self, &materialized) {
            Ok(()) => {
                self.inner.borrow_mut().last_materialized = Some(materialized);
                let hydrate_ms = Date::now() - hydrate_start;
                self.inner.borrow_mut().metrics.hydrate_ms = Some(hydrate_ms);
                Ok(())
            }
            Err(err) => {
                self.log_warn(&format!("Hydration failed: {:?}", err));
                renderer::update_dom_with_root(container_id, &materialized, Some(self))?;
                self.inner.borrow_mut().last_materialized = Some(materialized);
                let hydrate_ms = Date::now() - hydrate_start;
                self.inner.borrow_mut().metrics.hydrate_ms = Some(hydrate_ms);
                Ok(())
            }
        }
    }
}

impl LaplogosEngine {
    pub(crate) fn log_warn(&self, message: &str) {
        web_sys::console::warn_1(&JsValue::from_str(message));
        let mut inner = self.inner.borrow_mut();
        inner.error_log.push(format!("WARN: {}", message));
    }

    fn with_page<R>(&self, f: impl FnOnce(&LaplogosPage) -> R) -> R {
        let inner = self.inner.borrow();
        f(&inner.normalized_page)
    }

    pub(crate) fn dispatch_with_event(
        &self,
        action_json: &str,
        event_value: Option<String>,
    ) -> Result<(), JsValue> {
        if !self.allow_dispatch() {
            self.log_warn("dispatch rate limit exceeded");
            return Ok(());
        }

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

        self.log_action(&action);

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
            ValidatedAction::Nav { to } => {
                self.handle_nav(to);
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

                self.update_view("laplogos-root")
            }
        }
    }

    pub(crate) fn page_root(&self) -> LaplogosNode {
        self.with_page(|page| page.root.clone())
    }

    fn update_view(&self, container_id: &str) -> Result<(), JsValue> {
        let materialize_start = Date::now();
        let (materialized, warnings) = self.materialize_current()?;
        let materialize_ms = Date::now() - materialize_start;
        self.inner.borrow_mut().metrics.materialize_ms = Some(materialize_ms);
        for warn in warnings {
            self.log_warn(&warn);
        }

        let previous = self.inner.borrow().last_materialized.clone();
        if let Some(old) = previous {
            let diff_start = Date::now();
            let patches = diff::diff(&old, &materialized);
            let diff_ms = Date::now() - diff_start;
            self.inner.borrow_mut().metrics.diff_ms = Some(diff_ms);

            if patches.len() > MAX_PATCHES_PER_DISPATCH {
                let err = LaplogosError::new(format!(
                    "patch limit exceeded: {} > {}",
                    patches.len(),
                    MAX_PATCHES_PER_DISPATCH
                ));
                self.log_error(&err);
                return Err(JsValue::from_str(&err.message));
            }

            let patch_start = Date::now();
            if let Err(err) = renderer::apply_patches(self, container_id, &patches, &materialized) {
                self.log_warn(&format!("Patch apply failed: {:?}", err));
                renderer::update_dom_with_root(container_id, &materialized, Some(self))?;
            }
            let patch_ms = Date::now() - patch_start;
            self.inner.borrow_mut().metrics.patch_ms = Some(patch_ms);
        } else {
            let patch_start = Date::now();
            renderer::update_dom_with_root(container_id, &materialized, Some(self))?;
            let patch_ms = Date::now() - patch_start;
            self.inner.borrow_mut().metrics.patch_ms = Some(patch_ms);
        }

        self.inner.borrow_mut().last_materialized = Some(materialized);
        Ok(())
    }

    pub(crate) fn materialize_current(&self) -> Result<(LaplogosNode, Vec<String>), JsValue> {
        let mut warnings = Vec::new();
        let materialized = {
            let inner = self.inner.borrow();
            materialize_one_root(&inner.normalized_page.root, &inner.state, &mut warnings)
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
        let mut inner = self.inner.borrow_mut();
        inner.error_log.push(format!("ERROR: {}", err.message));
    }

    fn log_action(&self, action: &store::ActionSpec) {
        let text = serde_json::to_string(action).unwrap_or_else(|_| action.t.clone());
        let mut inner = self.inner.borrow_mut();
        inner.action_log.push(text);
    }

    fn allow_dispatch(&self) -> bool {
        let now = Date::now();
        let mut inner = self.inner.borrow_mut();
        let window_start = now - 1000.0;
        while let Some(oldest) = inner.dispatch_times.front() {
            if *oldest >= window_start {
                break;
            }
            inner.dispatch_times.pop_front();
        }
        if inner.dispatch_times.len() >= MAX_DISPATCH_PER_SEC {
            return false;
        }
        inner.dispatch_times.push_back(now);
        true
    }

    fn spawn_fetch(&self, url: String, on_success: Vec<Value>, on_fail: Vec<Value>) {
        let engine = self.clone();
        spawn_local(async move {
            let parsed = match web_sys::Url::new(&url) {
                Ok(url) => url,
                Err(_) => {
                    engine.log_warn("fetch.url is invalid");
                    engine.dispatch_actions(on_fail);
                    return;
                }
            };

            let host = parsed.host().to_lowercase();
            if host.is_empty() || !engine.is_host_allowed(&host) {
                engine.log_warn("fetch host is not allowed");
                engine.dispatch_actions(on_fail);
                return;
            }

            let window = match web_sys::window() {
                Some(window) => window,
                None => {
                    engine.log_warn("No window for fetch");
                    engine.dispatch_actions(on_fail);
                    return;
                }
            };

            let controller = web_sys::AbortController::new().ok();
            let mut opts = web_sys::RequestInit::new();
            opts.method("GET");
            opts.credentials(web_sys::RequestCredentials::Omit);
            if let Some(controller) = controller.as_ref() {
                opts.signal(Some(&controller.signal()));
            }

            let timeout_handle = if let Some(controller) = controller.as_ref() {
                let controller = controller.clone();
                let closure = Closure::wrap(Box::new(move || {
                    controller.abort();
                }) as Box<dyn FnMut()>);
                let handle = window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        FETCH_TIMEOUT_MS,
                    )
                    .ok();
                closure.forget();
                handle
            } else {
                None
            };

            let resp_value = JsFuture::from(window.fetch_with_str_and_init(&url, &opts)).await;
            if let Some(handle) = timeout_handle {
                window.clear_timeout_with_handle(handle);
            }

            let response = match resp_value {
                Ok(val) => match val.dyn_into::<web_sys::Response>() {
                    Ok(resp) => resp,
                    Err(_) => {
                        engine.log_warn("fetch response is invalid");
                        engine.dispatch_actions(on_fail);
                        return;
                    }
                },
                Err(_) => {
                    engine.log_warn("fetch failed");
                    engine.dispatch_actions(on_fail);
                    return;
                }
            };

            if !response.ok() {
                engine.log_warn("fetch response is not ok");
                engine.dispatch_actions(on_fail);
                return;
            }

            let final_url = response.url();
            if !final_url.is_empty() {
                match web_sys::Url::new(&final_url) {
                    Ok(final_parsed) => {
                        let final_host = final_parsed.host().to_lowercase();
                        if final_host.is_empty() || !engine.is_host_allowed(&final_host) {
                            engine.log_warn("fetch final host is not allowed");
                            engine.dispatch_actions(on_fail);
                            return;
                        }
                    }
                    Err(_) => {
                        engine.log_warn("fetch final url is invalid");
                        engine.dispatch_actions(on_fail);
                        return;
                    }
                }
            }

            let content_type = response
                .headers()
                .get("content-type")
                .ok()
                .flatten()
                .unwrap_or_default()
                .to_ascii_lowercase();
            if !content_type.contains("application/json") {
                engine.log_warn("fetch response must be application/json");
                engine.dispatch_actions(on_fail);
                return;
            }

            let text_promise = match response.text() {
                Ok(promise) => promise,
                Err(_) => {
                    engine.log_warn("fetch response text failed");
                    engine.dispatch_actions(on_fail);
                    return;
                }
            };
            let text_value = JsFuture::from(text_promise).await;
            let text = match text_value {
                Ok(val) => val.as_string().unwrap_or_default(),
                Err(_) => {
                    engine.log_warn("fetch response text failed");
                    engine.dispatch_actions(on_fail);
                    return;
                }
            };
            if text.len() > FETCH_MAX_JSON_BYTES {
                engine.log_warn("fetch response too large");
                engine.dispatch_actions(on_fail);
                return;
            }
            if JSON::parse(&text).is_err() {
                engine.log_warn("fetch response is not valid JSON");
                engine.dispatch_actions(on_fail);
                return;
            }

            engine.dispatch_actions(on_success);
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
            None => false,
        }
    }

    pub(crate) fn is_host_allowed(&self, host: &str) -> bool {
        let inner = self.inner.borrow();
        match inner.config.as_ref() {
            Some(config) => config.allows_host(host),
            None => false,
        }
    }

    fn handle_nav(&self, to: String) {
        if is_external_url(&to) {
            let parsed = match web_sys::Url::new(&to) {
                Ok(url) => url,
                Err(_) => {
                    self.log_warn("nav.to is invalid");
                    return;
                }
            };
            let host = parsed.host().to_lowercase();
            if host.is_empty() || !self.is_host_allowed(&host) {
                self.log_warn("nav host is not allowed");
                return;
            }
        }

        let callback = self.inner.borrow().on_nav.clone();
        match callback {
            Some(callback) => {
                let _ = callback.call1(&JsValue::NULL, &JsValue::from_str(&to));
            }
            None => self.log_warn("on_nav is not set"),
        }
    }
}

fn default_schema_version() -> u8 {
    SCHEMA_VERSION_DEFAULT
}

fn extract_schema_version(value: &Value) -> Result<(u8, bool), LaplogosError> {
    let obj = value
        .as_object()
        .ok_or_else(|| LaplogosError::new("page must be an object"))?;
    match obj.get("v") {
        None => Ok((SCHEMA_VERSION_DEFAULT, true)),
        Some(Value::Number(num)) => {
            let v = num
                .as_u64()
                .ok_or_else(|| LaplogosError::new("schema v must be an integer"))?;
            if v > u8::MAX as u64 {
                return Err(LaplogosError::new("schema v is out of range"));
            }
            Ok((v as u8, false))
        }
        Some(_) => Err(LaplogosError::new("schema v must be a number")),
    }
}

fn ensure_supported_schema_version(v: u8) -> Result<(), LaplogosError> {
    if v == SCHEMA_VERSION_DEFAULT {
        Ok(())
    } else {
        Err(LaplogosError::new(format!(
            "unsupported schema v: {}",
            v
        )))
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn render_laplogos(json_str: &str, container_id: &str) -> Result<(), JsValue> {
    let raw: Value =
        serde_json::from_str(json_str).map_err(|e| JsValue::from_str(&format!("Invalid JSON: {e}")))?;
    let (schema_v, v_missing) =
        extract_schema_version(&raw).map_err(|e| JsValue::from_str(&e.message))?;
    ensure_supported_schema_version(schema_v).map_err(|e| JsValue::from_str(&e.message))?;
    if v_missing {
        web_sys::console::warn_1(&JsValue::from_str("schema v is missing; defaulting to v1"));
    }
    let page = normalize_page(raw).map_err(|e| JsValue::from_str(&e.message))?;
    let state = LaplogosState::default();
    let mut warnings = Vec::new();
    let materialized =
        materialize_one_root(&page.root, &state, &mut warnings).map_err(|e| JsValue::from_str(&e.message))?;
    for warn in warnings {
        web_sys::console::warn_1(&JsValue::from_str(&warn));
    }

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("No document"))?;
    let container = document
        .get_element_by_id(container_id)
        .ok_or_else(|| JsValue::from_str("Container not found"))?;

    container.set_inner_html("");

    let root_el = renderer::render_node(&document, &materialized, None)?;
    container.append_child(&root_el)?;

    Ok(())
}

#[wasm_bindgen]
pub fn render_laplogos_to_string(page_json: JsValue, state_json: JsValue) -> Result<String, JsValue> {
    let page_value = js_value_to_json(page_json)?;
    let state_value = js_value_to_json(state_json)?;

    let (schema_v, _v_missing) =
        extract_schema_version(&page_value).map_err(|e| JsValue::from_str(&e.message))?;
    ensure_supported_schema_version(schema_v).map_err(|e| JsValue::from_str(&e.message))?;

    let page = normalize_page(page_value).map_err(|e| JsValue::from_str(&e.message))?;
    let state = parse_state(state_value)?;

    let mut warnings = Vec::new();
    let root =
        materialize_one_root(&page.root, &state, &mut warnings).map_err(|e| JsValue::from_str(&e.message))?;
    Ok(render_to_string(&root))
}

fn js_value_to_json(value: JsValue) -> Result<Value, JsValue> {
    let json = JSON::stringify(&value)
        .map_err(|_| JsValue::from_str("Failed to stringify JSON"))?
        .as_string()
        .ok_or_else(|| JsValue::from_str("Failed to stringify JSON"))?;
    serde_json::from_str(&json).map_err(|e| JsValue::from_str(&format!("Invalid JSON: {e}")))
}

fn parse_state(value: Value) -> Result<LaplogosState, JsValue> {
    match value {
        Value::Object(map) => Ok(LaplogosState {
            data: map.into_iter().collect(),
        }),
        _ => Err(JsValue::from_str("State JSON must be an object")),
    }
}

fn is_external_url(to: &str) -> bool {
    let lower = to.to_ascii_lowercase();
    lower.starts_with("http://") || lower.starts_with("https://")
}
