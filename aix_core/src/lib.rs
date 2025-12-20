use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

mod renderer;
mod store;
pub mod materialize;

use store::{apply_action, normalize_action, validate_action, AixState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AixPage {
    pub v: u8,
    pub root: AixNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AixNode {
    #[serde(rename = "i")]
    pub i: String,
    #[serde(rename = "t")]
    pub t: String,
    #[serde(rename = "p", default)]
    pub p: HashMap<String, Value>,
    #[serde(rename = "c", default)]
    pub c: Option<Vec<AixNode>>,
    #[serde(rename = "on", default)]
    pub on: Option<HashMap<String, Vec<Value>>>,
}

#[wasm_bindgen]
pub struct AixEngine {
    state: AixState,
    page: AixPage,
    logs: Vec<String>,
}

#[wasm_bindgen]
impl AixEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(initial_json: &str) -> Result<AixEngine, JsValue> {
        let page: AixPage = serde_json::from_str(initial_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid JSON: {e}")))?;

        Ok(AixEngine {
            state: AixState::default(),
            page,
            logs: Vec::new(),
        })
    }

    pub fn get_state_json(&self) -> String {
        serde_json::to_string(&self.state.data).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn dispatch(&mut self, action_json: &str) -> Result<(), JsValue> {
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

        let validated = match validate_action(action) {
            Ok(validated) => validated,
            Err(err) => {
                self.log_warn(&err);
                return Ok(());
            }
        };

        let changed = match apply_action(&mut self.state, validated, None) {
            Ok(changed) => changed,
            Err(err) => {
                self.log_warn(&err);
                return Ok(());
            }
        };

        if !changed {
            return Ok(());
        }

        self.rerender("aix-root")
    }

    pub fn rerender(&self, container_id: &str) -> Result<(), JsValue> {
        renderer::update_dom(self, container_id)
    }
}

impl AixEngine {
    fn log_warn(&mut self, message: &str) {
        web_sys::console::warn_1(&JsValue::from_str(message));
        self.logs.push(message.to_string());
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn render_aix(json_str: &str, container_id: &str) -> Result<(), JsValue> {
    let page: AixPage =
        serde_json::from_str(json_str).map_err(|e| JsValue::from_str(&format!("Invalid JSON: {e}")))?;

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("No document"))?;
    let container = document
        .get_element_by_id(container_id)
        .ok_or_else(|| JsValue::from_str("Container not found"))?;

    container.set_inner_html("");

    let root_el = renderer::render_node(&document, &page.root)?;
    container.append_child(&root_el)?;

    Ok(())
}
