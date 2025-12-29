use laplogos_core::diff::{diff, Patch};
use laplogos_core::materialize::is_valid_id;
use laplogos_core::LaplogosNode;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

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
fn patch_order_is_stable_and_ids_are_valid() {
    let mut old_root = base_node("root", "Box");
    old_root
        .p
        .insert("class".to_string(), Value::String("old".to_string()));
    old_root.c = Some(vec![txt_node("a", "one"), txt_node("b", "two")]);

    let mut new_root = base_node("root", "Box");
    new_root
        .p
        .insert("class".to_string(), Value::String("new".to_string()));
    let new_b = base_node("b", "Btn");
    new_root.c = Some(vec![new_b, txt_node("c", "three")]);

    let patches = diff(&old_root, &new_root);
    assert_eq!(patches.len(), 4);

    match &patches[0] {
        Patch::UpdateAttr { id, key, val } => {
            assert_eq!(id, "root");
            assert_eq!(key, "class");
            assert_eq!(val, "new");
        }
        other => panic!("unexpected patch order: {:?}", other),
    }
    match &patches[1] {
        Patch::Remove { id } => assert_eq!(id, "a"),
        other => panic!("unexpected patch order: {:?}", other),
    }
    match &patches[2] {
        Patch::Replace { id, .. } => assert_eq!(id, "b"),
        other => panic!("unexpected patch order: {:?}", other),
    }
    match &patches[3] {
        Patch::Append { parent_id, .. } => assert_eq!(parent_id, "root"),
        other => panic!("unexpected patch order: {:?}", other),
    }

    for patch in &patches {
        match patch {
            Patch::Append { parent_id, node } => {
                assert!(is_valid_id(parent_id));
                assert!(is_valid_id(&node.i));
            }
            Patch::Remove { id } => assert!(is_valid_id(id)),
            Patch::Replace { id, node } => {
                assert!(is_valid_id(id));
                assert!(is_valid_id(&node.i));
            }
            Patch::UpdateText { id, .. } => assert!(is_valid_id(id)),
            Patch::UpdateAttr { id, .. } => assert!(is_valid_id(id)),
            Patch::ReorderChildren { parent_id, ordered_ids } => {
                assert!(is_valid_id(parent_id));
                for id in ordered_ids {
                    assert!(is_valid_id(id));
                }
            }
        }
    }
}

#[test]
fn reorder_children_ids_are_unique_and_match_sets() {
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
    let Patch::ReorderChildren { ordered_ids, .. } = &patches[0] else {
        panic!("expected reorder patch");
    };

    let mut seen = HashSet::new();
    for id in ordered_ids {
        assert!(seen.insert(id));
    }

    let old_ids: HashSet<String> = old_root
        .c
        .as_ref()
        .unwrap()
        .iter()
        .map(|n| n.i.clone())
        .collect();
    let new_ids: HashSet<String> = new_root
        .c
        .as_ref()
        .unwrap()
        .iter()
        .map(|n| n.i.clone())
        .collect();
    let ordered_set: HashSet<String> = ordered_ids.iter().cloned().collect();

    assert_eq!(ordered_set, old_ids);
    assert_eq!(ordered_set, new_ids);
}
