use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, Element, Event, HtmlInputElement};

use crate::{diff::Patch, LaplogosEngine, LaplogosNode};

fn text_from_value(val: &serde_json::Value) -> String {
    val.as_str().map(|s| s.to_owned()).unwrap_or_else(|| val.to_string())
}

fn is_disallowed_href(href: &str) -> bool {
    let lower = href.trim().to_ascii_lowercase();
    lower.starts_with("javascript:") || lower.starts_with("data:")
}

pub fn render_node(
    document: &Document,
    node: &LaplogosNode,
    engine: Option<&LaplogosEngine>,
) -> Result<Element, JsValue> {
    let tag = match node.t.as_str() {
        "Box" => "div",
        "Txt" => "p",
        "Btn" => "button",
        "Inp" => "input",
        "Lnk" => "a",
        _ => "div",
    };

    let element = document.create_element(tag)?;
    element.set_attribute("data-laplogos-id", &node.i)?;

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
        "Inp" => {
            if let Some(val) = node.p.get("val") {
                element.set_attribute("value", &text_from_value(val))?;
            }
            if let Some(engine) = engine {
                if let Some(key) = node.bind_key.as_ref() {
                    let action_json = serde_json::json!({
                        "t": "set",
                        "d": { "key": key, "val_from": "event.value" }
                    })
                    .to_string();
                    attach_event_listener(&element, "input", vec![action_json], engine)?;
                } else {
                    engine.log_warn("Inp.p.val must be $key");
                }
            }
        }
        "Lnk" => {
            if let Some(href) = node.p.get("href") {
                let href_text = text_from_value(href);
                if is_disallowed_href(&href_text) {
                    if let Some(engine) = engine {
                        engine.log_warn("Lnk.p.href is not allowed");
                    }
                } else {
                    element.set_attribute("href", &href_text)?;
                    if let Some(engine) = engine {
                        attach_nav_listener(&element, &href_text, engine)?;
                    }
                }
            }
            if let Some(label) = node.p.get("label") {
                element.set_text_content(Some(&text_from_value(label)));
            }
        }
        _ => {}
    }

    if let Some(engine) = engine {
        if let Some(on) = node.on.as_ref() {
            for (event_name, actions) in on {
                let mut action_jsons = Vec::new();
                for action in actions {
                    if let Ok(text) = serde_json::to_string(action) {
                        action_jsons.push(text);
                    }
                }
                if !action_jsons.is_empty() {
                    attach_event_listener(&element, event_name, action_jsons, engine)?;
                }
            }
        }
    }

    if let Some(children) = node.c.as_ref() {
        for child in children {
            let child_el = render_node(document, child, engine)?;
            element.append_child(&child_el)?;
        }
    }

    Ok(element)
}

pub fn update_dom_with_root(
    container_id: &str,
    root: &LaplogosNode,
    engine: Option<&LaplogosEngine>,
) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("No document"))?;
    let container = document
        .get_element_by_id(container_id)
        .ok_or_else(|| JsValue::from_str("Container not found"))?;

    container.set_inner_html("");
    let root_el = render_node(&document, root, engine)?;
    root_el.set_attribute("data-laplogos-root", "1")?;
    container.append_child(&root_el)?;
    Ok(())
}

pub fn bind_events(engine: &LaplogosEngine, root: &LaplogosNode) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("No document"))?;
    bind_events_for_node(&document, root, engine)
}

fn bind_events_for_node(
    document: &Document,
    node: &LaplogosNode,
    engine: &LaplogosEngine,
) -> Result<(), JsValue> {
    let element = find_by_id(document, &node.i)?;

    match node.t.as_str() {
        "Inp" => {
            if let Some(key) = node.bind_key.as_ref() {
                let action_json = serde_json::json!({
                    "t": "set",
                    "d": { "key": key, "val_from": "event.value" }
                })
                .to_string();
                attach_event_listener(&element, "input", vec![action_json], engine)?;
            } else {
                engine.log_warn("Inp.p.val must be $key");
            }
        }
        "Lnk" => {
            if let Some(href) = node.p.get("href") {
                let href_text = text_from_value(href);
                if is_disallowed_href(&href_text) {
                    engine.log_warn("Lnk.p.href is not allowed");
                } else {
                    attach_nav_listener(&element, &href_text, engine)?;
                }
            }
        }
        _ => {}
    }

    if let Some(on) = node.on.as_ref() {
        for (event_name, actions) in on {
            let mut action_jsons = Vec::new();
            for action in actions {
                if let Ok(text) = serde_json::to_string(action) {
                    action_jsons.push(text);
                }
            }
            if !action_jsons.is_empty() {
                attach_event_listener(&element, event_name, action_jsons, engine)?;
            }
        }
    }

    if let Some(children) = node.c.as_ref() {
        for child in children {
            bind_events_for_node(document, child, engine)?;
        }
    }

    Ok(())
}

fn attach_event_listener(
    element: &Element,
    event_name: &str,
    actions: Vec<String>,
    engine: &LaplogosEngine,
) -> Result<(), JsValue> {
    let engine = engine.clone();
    let event_name = event_name.to_string();
    let closure = Closure::wrap(Box::new(move |event: Event| {
        let event_value = extract_event_value(&event);
        for action in &actions {
            if let Err(err) = engine.dispatch_with_event(action, event_value.clone()) {
                web_sys::console::warn_1(&err);
            }
        }
    }) as Box<dyn FnMut(_)>);

    element.add_event_listener_with_callback(&event_name, closure.as_ref().unchecked_ref())?;
    closure.forget();
    Ok(())
}

fn attach_nav_listener(
    element: &Element,
    href: &str,
    engine: &LaplogosEngine,
) -> Result<(), JsValue> {
    let engine = engine.clone();
    let href = href.to_string();
    let closure = Closure::wrap(Box::new(move |event: Event| {
        event.prevent_default();
        let action = serde_json::json!({
            "t": "nav",
            "d": { "to": href }
        })
        .to_string();
        if let Err(err) = engine.dispatch_with_event(&action, None) {
            web_sys::console::warn_1(&err);
        }
    }) as Box<dyn FnMut(_)>);

    element.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();
    Ok(())
}

fn extract_event_value(event: &Event) -> Option<String> {
    event
        .target()
        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
        .map(|input| input.value())
}

pub fn apply_patches(
    engine: &LaplogosEngine,
    _container_id: &str,
    patches: &[Patch],
    _new_root: &LaplogosNode,
) -> Result<(), JsValue> {
    if patches.is_empty() {
        return Ok(());
    }

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("No document"))?;

    for patch in patches {
        match patch {
            Patch::Append { parent_id, node } => {
                let parent = find_by_id(&document, parent_id)?;
                let child = render_node(&document, node, Some(engine))?;
                parent.append_child(&child)?;
            }
            Patch::Remove { id } => {
                let target = find_by_id(&document, id)?;
                target.remove();
            }
            Patch::Replace { id, node } => {
                let target = find_by_id(&document, id)?;
                let replacement = render_node(&document, node, Some(engine))?;
                target.replace_with_with_node_1(&replacement)?;
            }
            Patch::UpdateText { id, val } => {
                let target = find_by_id(&document, id)?;
                target.set_text_content(Some(val));
            }
            Patch::UpdateAttr { id, key, val } => {
                let target = find_by_id(&document, id)?;
                if key == "value" {
                    if let Some(active) = document.active_element() {
                        if target.is_same_node(Some(&active)) {
                            continue;
                        }
                    }
                }
                target.set_attribute(key, val)?;
            }
            Patch::RemoveAttr { id, key } => {
                let target = find_by_id(&document, id)?;
                target.remove_attribute(key)?;
            }
            Patch::ReorderChildren {
                parent_id,
                ordered_ids,
            } => {
                let parent = find_by_id(&document, parent_id)?;
                for child_id in ordered_ids {
                    let child = find_by_id(&document, child_id)?;
                    parent.append_child(&child)?;
                }
            }
        }
    }

    Ok(())
}

fn find_by_id(document: &Document, id: &str) -> Result<Element, JsValue> {
    let selector = format!("[data-laplogos-id=\"{}\"]", id);
    document
        .query_selector(&selector)?
        .ok_or_else(|| JsValue::from_str(&format!("Element not found: {}", id)))
}

pub fn render_to_string(node: &LaplogosNode) -> String {
    let tag = match node.t.as_str() {
        "Box" => "div",
        "Txt" => "p",
        "Btn" => "button",
        "Inp" => "input",
        "Lnk" => "a",
        _ => "div",
    };

    let mut attrs = vec![format!(
        "data-laplogos-id=\"{}\"",
        escape_attr(&node.i)
    )];

    match node.t.as_str() {
        "Inp" => {
            if let Some(val) = node.p.get("val") {
                attrs.push(format!("value=\"{}\"", escape_attr(&text_from_value(val))));
            }
        }
        "Lnk" => {
            if let Some(href) = node.p.get("href") {
                let href_text = text_from_value(href);
                if !is_disallowed_href(&href_text) {
                    attrs.push(format!("href=\"{}\"", escape_attr(&href_text)));
                }
            }
        }
        _ => {}
    }

    let attr_text = if attrs.is_empty() {
        "".to_string()
    } else {
        format!(" {}", attrs.join(" "))
    };

    if tag == "input" {
        return format!("<{tag}{attr_text}/>");
    }

    let mut inner = String::new();
    match node.t.as_str() {
        "Txt" => {
            if let Some(val) = node.p.get("val") {
                inner.push_str(&escape_text(&text_from_value(val)));
            }
        }
        "Btn" => {
            if let Some(label) = node.p.get("label") {
                inner.push_str(&escape_text(&text_from_value(label)));
            }
        }
        "Lnk" => {
            if let Some(label) = node.p.get("label") {
                inner.push_str(&escape_text(&text_from_value(label)));
            }
        }
        _ => {}
    }

    if let Some(children) = node.c.as_ref() {
        for child in children {
            inner.push_str(&render_to_string(child));
        }
    }

    format!("<{tag}{attr_text}>{inner}</{tag}>")
}

fn escape_text(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_attr(text: &str) -> String {
    escape_text(text).replace('"', "&quot;")
}
