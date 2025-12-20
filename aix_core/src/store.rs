use serde::Deserialize;
use serde_json::{Number, Value};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct AixState {
    pub data: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ActionSpec {
    pub t: String,
    #[serde(default)]
    pub d: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum ValidatedAction {
    Set { key: String, val: ValueSource },
    Inc { key: String, by: f64 },
}

#[derive(Debug, Clone)]
pub enum ValueSource {
    Direct(Value),
    EventValue,
}

pub fn normalize_action(raw: &Value) -> Result<ActionSpec, String> {
    match raw {
        Value::String(t) => Ok(ActionSpec {
            t: t.to_lowercase(),
            d: HashMap::new(),
        }),
        Value::Object(map) => {
            let t = map
                .get("t")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "ActionSpec.t must be a string".to_string())?
                .to_lowercase();
            let d = match map.get("d") {
                None => HashMap::new(),
                Some(Value::Object(obj)) => obj.clone().into_iter().collect(),
                Some(_) => return Err("ActionSpec.d must be an object".to_string()),
            };
            Ok(ActionSpec { t, d })
        }
        _ => Err("ActionSpec must be string or object".to_string()),
    }
}

pub fn validate_action(action: ActionSpec) -> Result<ValidatedAction, String> {
    match action.t.as_str() {
        "set" => {
            let key = action
                .d
                .get("key")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "set.key must be a string".to_string())?
                .to_string();
            let has_val = action.d.get("val").is_some();
            let val_from = action.d.get("val_from");

            if has_val == val_from.is_some() {
                return Err("set requires exactly one of val or val_from".to_string());
            }

            if let Some(val_from) = val_from {
                let val_from = val_from
                    .as_str()
                    .ok_or_else(|| "set.val_from must be a string".to_string())?;
                if val_from != "event.value" {
                    return Err("set.val_from only allows event.value".to_string());
                }
                Ok(ValidatedAction::Set {
                    key,
                    val: ValueSource::EventValue,
                })
            } else {
                let val = action
                    .d
                    .get("val")
                    .cloned()
                    .ok_or_else(|| "set.val is required".to_string())?;
                Ok(ValidatedAction::Set {
                    key,
                    val: ValueSource::Direct(val),
                })
            }
        }
        "inc" => {
            let key = action
                .d
                .get("key")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "inc.key must be a string".to_string())?
                .to_string();
            let by = match action.d.get("by") {
                None => 1.0,
                Some(v) => v
                    .as_f64()
                    .ok_or_else(|| "inc.by must be a number".to_string())?,
            };
            Ok(ValidatedAction::Inc { key, by })
        }
        "fetch" => Err("fetch is not supported in this phase".to_string()),
        _ => Err(format!("Unknown action type: {}", action.t)),
    }
}

pub fn apply_action(
    state: &mut AixState,
    action: ValidatedAction,
    event_value: Option<String>,
) -> Result<bool, String> {
    match action {
        ValidatedAction::Set { key, val } => {
            let new_val = match val {
                ValueSource::Direct(v) => v,
                ValueSource::EventValue => Value::String(
                    event_value.ok_or_else(|| "event.value is missing".to_string())?,
                ),
            };

            let existing = state.data.get(&key);
            if existing == Some(&new_val) {
                return Ok(false);
            }
            state.data.insert(key, new_val);
            Ok(true)
        }
        ValidatedAction::Inc { key, by } => {
            let current = state
                .data
                .get(&key)
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let next = current + by;
            let number = Number::from_f64(next)
                .ok_or_else(|| "inc resulted in non-finite number".to_string())?;
            state.data.insert(key, Value::Number(number));
            Ok(true)
        }
    }
}
