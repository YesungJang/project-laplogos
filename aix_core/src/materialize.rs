use serde_json::Value;

use crate::{store::AixState, AixNode};

#[derive(Debug, Clone, Default)]
pub struct MaterializeScope {
    pub item: Option<String>,
    pub key: Option<String>,
}

pub fn resolve_text(val: &Value, state: &AixState, scope: &MaterializeScope) -> String {
    let raw = match val.as_str() {
        Some(s) => s,
        None => return val.to_string(),
    };

    if raw == "$item" {
        if let Some(item) = scope.item.as_ref() {
            return item.clone();
        }
    }

    if raw == "$key" {
        if let Some(key) = scope.key.as_ref() {
            return key.clone();
        }
    }

    if let Some(stripped) = raw.strip_prefix('$') {
        if let Some(value) = state.data.get(stripped) {
            if let Some(text) = value.as_str() {
                return text.to_string();
            }
            return value.to_string();
        }
    }

    raw.to_string()
}

pub fn resolve_if(val: &Value, state: &AixState) -> (bool, Option<String>) {
    let key = match val.as_str() {
        Some(s) => s,
        None => return (false, Some("if must be a string key".to_string())),
    };

    let key = key.strip_prefix('$').unwrap_or(key);
    match state.data.get(key) {
        Some(Value::Bool(true)) => (true, None),
        Some(Value::Bool(false)) => (false, None),
        Some(_) => (false, Some(format!("if key '{}' is not boolean", key))),
        None => (false, None),
    }
}

pub fn is_valid_id(id: &str) -> bool {
    if id.is_empty() {
        return false;
    }
    id.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ':' || c == '-')
}

pub fn validate_global_ids(node: &AixNode) -> Result<(), String> {
    if !is_valid_id(&node.i) {
        return Err(format!("Invalid id: {}", node.i));
    }
    if let Some(children) = node.c.as_ref() {
        for child in children {
            validate_global_ids(child)?;
        }
    }
    Ok(())
}
