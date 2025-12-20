use serde_json::Value;

use crate::store::AixState;

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
