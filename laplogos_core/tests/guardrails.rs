use std::collections::HashMap;

use laplogos_core::{materialize::materialize_one_root, LaplogosNode, LaplogosState};
use serde_json::Value;

fn box_node(id: String, child: Option<LaplogosNode>) -> LaplogosNode {
    LaplogosNode {
        i: id,
        t: "Box".to_string(),
        p: HashMap::new(),
        c: child.map(|node| vec![node]),
        if_: None,
        for_: None,
        on: None,
        r: None,
        st: None,
        o: None,
    }
}

fn make_chain(count: usize) -> LaplogosNode {
    let mut current = box_node(format!("n{}", count - 1), None);
    for idx in (0..count - 1).rev() {
        current = box_node(format!("n{}", idx), Some(current));
    }
    current
}

#[test]
fn guardrail_max_nodes() {
    let root = make_chain(5001);
    let mut warnings = Vec::new();
    let state = LaplogosState::default();
    let result = materialize_one_root(&root, &state, &mut warnings);
    assert!(result.is_err());
}

#[test]
fn guardrail_loop_limit() {
    let items: Vec<Value> = (0..201)
        .map(|i| Value::String(format!("item{}", i)))
        .collect();
    let loop_node = LaplogosNode {
        i: "item".to_string(),
        t: "Txt".to_string(),
        p: HashMap::new(),
        c: None,
        if_: None,
        for_: Some(Value::Array(items)),
        on: None,
        r: None,
        st: None,
        o: None,
    };
    let root = LaplogosNode {
        i: "root".to_string(),
        t: "Box".to_string(),
        p: HashMap::new(),
        c: Some(vec![loop_node]),
        if_: None,
        for_: None,
        on: None,
        r: None,
        st: None,
        o: None,
    };
    let mut warnings = Vec::new();
    let state = LaplogosState::default();
    let result = materialize_one_root(&root, &state, &mut warnings);
    assert!(result.is_err());
}
