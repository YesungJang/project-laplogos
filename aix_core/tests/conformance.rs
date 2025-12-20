use std::collections::HashMap;

use aix_core::{materialize::validate_global_ids, AixNode};

fn node(id: &str, children: Option<Vec<AixNode>>) -> AixNode {
    AixNode {
        i: id.to_string(),
        t: "Box".to_string(),
        p: HashMap::new(),
        c: children,
        on: None,
    }
}

#[test]
fn valid_global_ids_pass() {
    let tree = node(
        "root_ok",
        Some(vec![node("child-1", None), node("child:2", None)]),
    );
    assert!(validate_global_ids(&tree).is_ok());
}

#[test]
fn invalid_global_ids_fail() {
    let tree = node("root bad", None);
    assert!(validate_global_ids(&tree).is_err());
}
