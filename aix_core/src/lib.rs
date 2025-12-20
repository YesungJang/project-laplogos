use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

mod renderer;
mod store;

use store::{reduce, Action, AixState};

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
    pub on: Option<HashMap<String, Vec<String>>>,
}

#[wasm_bindgen]
pub struct AixEngine {
    state: AixState,
    page: AixPage,
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
        })
    }

    pub fn get_state_json(&self) -> String {
        serde_json::to_string(&self.state.data).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn dispatch(&mut self, action_json: &str) -> Result<(), JsValue> {
        let action: Action =
            serde_json::from_str(action_json).map_err(|e| JsValue::from_str(&format!("Invalid action JSON: {e}")))?;
        reduce(&mut self.state, action);
        Ok(())
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
