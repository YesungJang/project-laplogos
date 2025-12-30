use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

use crate::materialize::LaplogosError;
use crate::{LaplogosNode, LaplogosPage};

fn default_schema_version() -> u8 {
    1
}

#[derive(Debug, Clone, Deserialize)]
struct PackedPage {
    #[serde(default = "default_schema_version")]
    pub v: u8,
    pub root: PackedNode,
    #[serde(default)]
    pub defs: HashMap<String, PackedNode>,
    #[serde(default)]
    pub styles: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize)]
struct PackedNode {
    #[serde(rename = "i")]
    pub i: Option<String>,
    #[serde(rename = "t")]
    pub t: Option<String>,
    #[serde(rename = "p", default)]
    pub p: HashMap<String, Value>,
    #[serde(rename = "c", default)]
    pub c: Option<Vec<PackedNode>>,
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

pub fn normalize_page(input: Value) -> Result<LaplogosPage, LaplogosError> {
    let Value::Object(obj) = input else {
        return Err(LaplogosError::new("page must be an object"));
    };
    let is_packed = obj.contains_key("defs") || obj.contains_key("styles");

    if !is_packed {
        let mut page: LaplogosPage = serde_json::from_value(Value::Object(obj))
            .map_err(|e| LaplogosError::new(format!("Invalid page JSON: {e}")))?;
        page.defs = None;
        page.styles = None;
        strip_packed_fields(&mut page.root);
        return Ok(page);
    }

    if let Some(styles_value) = obj.get("styles") {
        validate_styles(styles_value)?;
    }

    let packed: PackedPage = serde_json::from_value(Value::Object(obj))
        .map_err(|e| LaplogosError::new(format!("Invalid packed page JSON: {e}")))?;

    let root = normalize_node(&packed.root, &packed.defs, &packed.styles, None)?;
    Ok(LaplogosPage {
        v: packed.v,
        root,
        defs: None,
        styles: None,
    })
}

fn normalize_node(
    node: &PackedNode,
    defs: &HashMap<String, PackedNode>,
    styles: &HashMap<String, HashMap<String, String>>,
    override_id: Option<String>,
) -> Result<LaplogosNode, LaplogosError> {
    let effective_id = override_id.or_else(|| node.i.clone());

    if let Some(ref_id) = node.r.as_ref() {
        if effective_id.is_none() {
            return Err(LaplogosError::new("ref node requires i"));
        }
        let def = defs
            .get(ref_id)
            .ok_or_else(|| LaplogosError::new(format!("Undefined ref: {}", ref_id)))?;
        let mut normalized = normalize_node(def, defs, styles, effective_id)?;
        apply_node_overrides(&mut normalized, node, defs, styles)?;
        return Ok(normalized);
    }

    let id = effective_id.ok_or_else(|| LaplogosError::new("node.i is required"))?;
    let t = node
        .t
        .clone()
        .ok_or_else(|| LaplogosError::new("node.t is required"))?;

    let mut normalized = LaplogosNode {
        i: id,
        t,
        p: node.p.clone(),
        bind_key: None,
        c: None,
        if_: node.if_.clone(),
        for_: node.for_.clone(),
        on: node.on.clone(),
        r: None,
        st: None,
        o: None,
    };

    if let Some(children) = node.c.as_ref() {
        let mut out = Vec::with_capacity(children.len());
        for child in children {
            out.push(normalize_node(child, defs, styles, None)?);
        }
        normalized.c = Some(out);
    }

    apply_prop_overrides(&mut normalized, node, styles)?;

    Ok(normalized)
}

fn apply_node_overrides(
    normalized: &mut LaplogosNode,
    node: &PackedNode,
    defs: &HashMap<String, PackedNode>,
    styles: &HashMap<String, HashMap<String, String>>,
) -> Result<(), LaplogosError> {
    if node.if_.is_some() {
        normalized.if_ = node.if_.clone();
    }
    if node.for_.is_some() {
        normalized.for_ = node.for_.clone();
    }
    if node.on.is_some() {
        normalized.on = node.on.clone();
    }
    if let Some(children) = node.c.as_ref() {
        let mut out = Vec::with_capacity(children.len());
        for child in children {
            out.push(normalize_node(child, defs, styles, None)?);
        }
        normalized.c = Some(out);
    }

    apply_prop_overrides(normalized, node, styles)?;
    Ok(())
}

fn apply_prop_overrides(
    normalized: &mut LaplogosNode,
    node: &PackedNode,
    styles: &HashMap<String, HashMap<String, String>>,
) -> Result<(), LaplogosError> {
    if let Some(override_props) = node.o.as_ref() {
        validate_override(override_props)?;
        for (key, val) in override_props {
            normalized.p.insert(key.clone(), val.clone());
        }
    }

    if let Some(style_id) = node.st.as_ref() {
        let style = styles
            .get(style_id)
            .ok_or_else(|| LaplogosError::new(format!("Undefined style: {}", style_id)))?;
        for (key, val) in style {
            normalized
                .p
                .entry(key.clone())
                .or_insert_with(|| Value::String(val.clone()));
        }
    }

    Ok(())
}

fn validate_styles(styles_value: &Value) -> Result<(), LaplogosError> {
    let styles_obj = styles_value
        .as_object()
        .ok_or_else(|| LaplogosError::new("styles must be an object"))?;
    for (style_id, style_val) in styles_obj {
        let style_obj = style_val.as_object().ok_or_else(|| {
            LaplogosError::new(format!("styles.{} must be an object", style_id))
        })?;
        for (key, val) in style_obj {
            if !val.is_string() {
                return Err(LaplogosError::new(format!(
                    "styles.{}.{key} must be a string",
                    style_id
                )));
            }
        }
    }
    Ok(())
}

fn validate_override(override_props: &HashMap<String, Value>) -> Result<(), LaplogosError> {
    for key in override_props.keys() {
        if is_reserved_override_key(key) {
            return Err(LaplogosError::new(format!(
                "override cannot change non-prop field: {}",
                key
            )));
        }
    }
    Ok(())
}

fn is_reserved_override_key(key: &str) -> bool {
    matches!(key, "i" | "t" | "p" | "c" | "if" | "for" | "on" | "r" | "st" | "o")
}

fn strip_packed_fields(node: &mut LaplogosNode) {
    node.r = None;
    node.st = None;
    node.o = None;
    if let Some(children) = node.c.as_mut() {
        for child in children {
            strip_packed_fields(child);
        }
    }
}
