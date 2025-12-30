use laplogos_core::diff::{diff, Patch};
use laplogos_core::LaplogosNode;
use serde_json::Value;
use std::collections::HashMap;

fn base_node(id: &str, t: &str) -> LaplogosNode {
    LaplogosNode {
        i: id.to_string(),
        t: t.to_string(),
        p: HashMap::new(),
        c: None,
        if_: None,
        for_: None,
        on: None,
        r: None,
        st: None,
        o: None,
    }
}

fn txt_node(id: &str, val: &str) -> LaplogosNode {
    let mut node = base_node(id, "Txt");
    node.p
        .insert("val".to_string(), Value::String(val.to_string()));
    node
}

#[test]
fn diff_updates_text_value() {
    let old = txt_node("t1", "a");
    let new = txt_node("t1", "b");

    let patches = diff(&old, &new);
    assert_eq!(patches.len(), 1);
    match &patches[0] {
        Patch::UpdateText { id, val } => {
            assert_eq!(id, "t1");
            assert_eq!(val, "b");
        }
        other => panic!("unexpected patch: {:?}", other),
    }
}

#[test]
fn diff_emits_reorder_children() {
    let mut old_root = base_node("root", "Box");
    old_root.c = Some(vec![
        base_node("a", "Box"),
        base_node("b", "Box"),
        base_node("c", "Box"),
    ]);

    let mut new_root = base_node("root", "Box");
    new_root.c = Some(vec![
        base_node("c", "Box"),
        base_node("b", "Box"),
        base_node("a", "Box"),
    ]);

    let patches = diff(&old_root, &new_root);
    assert_eq!(patches.len(), 1);
    match &patches[0] {
        Patch::ReorderChildren {
            parent_id,
            ordered_ids,
        } => {
            assert_eq!(parent_id, "root");
            assert_eq!(ordered_ids, &vec!["c".to_string(), "b".to_string(), "a".to_string()]);
        }
        other => panic!("unexpected patch: {:?}", other),
    }
}
