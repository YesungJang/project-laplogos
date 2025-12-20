use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

use crate::{AixEngine, AixNode};

fn text_from_value(val: &serde_json::Value) -> String {
    val.as_str().map(|s| s.to_owned()).unwrap_or_else(|| val.to_string())
}

pub fn render_node(document: &Document, node: &AixNode) -> Result<Element, JsValue> {
    let tag = match node.t.as_str() {
        "Box" => "div",
        "Txt" => "p",
        "Btn" => "button",
        _ => "div",
    };

    let element = document.create_element(tag)?;
    element.set_attribute("id", &node.i)?;

    match node.t.as_str() {
        "Txt" => {
            if let Some(val) = node.p.get("val") {
                element.set_text_content(Some(&text_from_value(val)));
            }
        }
        "Btn" => {
            if let Some(label) = node.p.get("label") {
                element.set_text_content(Some(&text_from_value(label)));
            }
        }
        _ => {}
    }

    if let Some(children) = node.c.as_ref() {
        for child in children {
            let child_el = render_node(document, child)?;
            element.append_child(&child_el)?;
        }
    }

    Ok(element)
}

pub fn update_dom(engine: &AixEngine, container_id: &str) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("No document"))?;
    let container = document
        .get_element_by_id(container_id)
        .ok_or_else(|| JsValue::from_str("Container not found"))?;

    container.set_inner_html("");
    let root_el = render_node(&document, &engine.page.root)?;
    container.append_child(&root_el)?;
    Ok(())
}
