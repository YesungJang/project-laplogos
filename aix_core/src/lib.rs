use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

mod renderer;

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

pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

pub fn render_aix(json_str: &str, container_id: &str) -> Result<(), wasm_bindgen::JsValue> {
    let page: AixPage = serde_json::from_str(json_str)
        .map_err(|e| wasm_bindgen::JsValue::from_str(&format!("Invalid JSON: {e}")))?;

    let window = web_sys::window().ok_or_else(|| wasm_bindgen::JsValue::from_str("No window"))?;
    let document = window
        .document()
        .ok_or_else(|| wasm_bindgen::JsValue::from_str("No document"))?;
    let container = document
        .get_element_by_id(container_id)
        .ok_or_else(|| wasm_bindgen::JsValue::from_str("Container not found"))?;

    container.set_inner_html("");

    let root_el = renderer::render_node(&document, &page.root)?;
    container.append_child(&root_el)?;

    Ok(())
}
