use laplogos_core::diff::{diff, Patch};
use laplogos_core::LaplogosNode;
use serde_json::Value;
use std::collections::HashMap;

fn base_node(id: &str, t: &str) -> LaplogosNode {
    LaplogosNode {
        i: id.to_string(),
        t: t.to_string(),
        p: HashMap::new(),
        bind_key: None,
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

fn lnk_node(id: &str, href: &str) -> LaplogosNode {
    let mut node = base_node(id, "Lnk");
    node.p
        .insert("href".to_string(), Value::String(href.to_string()));
    node
}

fn inp_node(id: &str, bind_key: &str) -> LaplogosNode {
    let mut node = base_node(id, "Inp");
    node.bind_key = Some(bind_key.to_string());
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
fn diff_emits_remove_attr_when_missing() {
    let mut old = base_node("root", "Box");
    old.p
        .insert("class".to_string(), Value::String("old".to_string()));
    let new = base_node("root", "Box");

    let patches = diff(&old, &new);
    assert_eq!(patches.len(), 1);
    match &patches[0] {
        Patch::RemoveAttr { id, key } => {
            assert_eq!(id, "root");
            assert_eq!(key, "class");
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

#[test]
fn diff_reorders_with_inserted_children() {
    let mut old_root = base_node("root", "Box");
    old_root.c = Some(vec![
        base_node("a", "Box"),
        base_node("b", "Box"),
        base_node("c", "Box"),
    ]);

    let mut new_root = base_node("root", "Box");
    new_root.c = Some(vec![
        base_node("a", "Box"),
        base_node("x", "Box"),
        base_node("b", "Box"),
        base_node("c", "Box"),
    ]);

    let patches = diff(&old_root, &new_root);
    assert_eq!(patches.len(), 2);
    match &patches[0] {
        Patch::Append { parent_id, node } => {
            assert_eq!(parent_id, "root");
            assert_eq!(node.i, "x");
        }
        other => panic!("unexpected patch: {:?}", other),
    }
    match &patches[1] {
        Patch::ReorderChildren {
            parent_id,
            ordered_ids,
        } => {
            assert_eq!(parent_id, "root");
            assert_eq!(
                ordered_ids,
                &vec![
                    "a".to_string(),
                    "x".to_string(),
                    "b".to_string(),
                    "c".to_string()
                ]
            );
        }
        other => panic!("unexpected patch: {:?}", other),
    }
}

#[test]
fn diff_replaces_link_when_href_changes() {
    let old = lnk_node("link", "/a");
    let new = lnk_node("link", "/b");

    let patches = diff(&old, &new);
    assert_eq!(patches.len(), 1);
    match &patches[0] {
        Patch::Replace { id, node } => {
            assert_eq!(id, "link");
            assert_eq!(node.t, "Lnk");
        }
        other => panic!("unexpected patch: {:?}", other),
    }
}

#[test]
fn diff_replaces_input_when_bind_key_changes() {
    let old = inp_node("field", "a");
    let new = inp_node("field", "b");

    let patches = diff(&old, &new);
    assert_eq!(patches.len(), 1);
    match &patches[0] {
        Patch::Replace { id, node } => {
            assert_eq!(id, "field");
            assert_eq!(node.t, "Inp");
        }
        other => panic!("unexpected patch: {:?}", other),
    }
}
