use wasm_bindgen::JsValue;
use web_sys::{Document, Element};

use crate::AixNode;

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
