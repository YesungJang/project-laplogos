use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct AixState {
    pub data: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum Action {
    Set { key: String, val: String },
}

pub fn reduce(state: &mut AixState, action: Action) {
    match action {
        Action::Set { key, val } => {
            state.data.insert(key, val);
        }
    }
}
