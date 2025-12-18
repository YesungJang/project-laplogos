use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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
