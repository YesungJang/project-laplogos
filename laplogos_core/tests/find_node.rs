use laplogos_core::materialize::{find_node_snapshot_json, find_node_snapshot_value};
use laplogos_core::{normalize_page, LaplogosNode, LaplogosState};
use serde_json::{json, Value};

/// A page exercising state substitution, `if`, loop expansion and nested props.
fn demo_root() -> LaplogosNode {
    let raw = json!({
        "v": 1,
        "root": {
            "i": "root",
            "t": "Box",
            "c": [
                { "i": "header", "t": "Txt", "p": { "val": "$title" } },
                { "i": "hidden", "t": "Txt", "p": { "val": "nope" }, "if": "$show" },
                {
                    "i": "meta",
                    "t": "Box",
                    "p": { "z": 1, "cfg": { "y": true, "a": [3, 1, 2] } }
                },
                {
                    "i": "row",
                    "t": "Box",
                    "for": ["a", "b"],
                    "c": [{ "i": "label", "t": "Txt", "p": { "val": "$item" } }]
                }
            ]
        }
    });
    normalize_page(raw).expect("normalize page").root
}

fn root_from(raw: Value) -> LaplogosNode {
    normalize_page(raw).expect("normalize page").root
}

fn state_from(pairs: &[(&str, Value)]) -> LaplogosState {
    let mut state = LaplogosState::default();
    for (key, val) in pairs {
        state.data.insert((*key).to_string(), val.clone());
    }
    state
}

fn demo_state() -> LaplogosState {
    state_from(&[("title", json!("Hello")), ("show", json!(false))])
}

fn child_ids(snapshot: &str) -> Vec<String> {
    let value: Value = serde_json::from_str(snapshot).expect("parse snapshot");
    value["c"]
        .as_array()
        .expect("children array")
        .iter()
        .map(|child| child["i"].as_str().expect("child id").to_string())
        .collect()
}

#[test]
fn finds_root_by_exact_id() {
    let root = demo_root();
    let state = demo_state();

    let snapshot = find_node_snapshot_json(&root, &state, "root").expect("find root");
    let value: Value = serde_json::from_str(&snapshot).expect("parse snapshot");

    assert_eq!(value["i"], json!("root"));
    assert_eq!(value["t"], json!("Box"));
}

#[test]
fn finds_nested_node_with_current_state_substitution() {
    let root = demo_root();

    let snapshot =
        find_node_snapshot_json(&root, &demo_state(), "header").expect("find header");
    assert_eq!(
        snapshot,
        r#"{"c":null,"for":null,"i":"header","if":null,"o":null,"on":null,"p":{"val":"Hello"},"r":null,"st":null,"t":"Txt"}"#
    );

    // The lookup reflects the CURRENT state, not the input schema.
    let other = state_from(&[("title", json!("World")), ("show", json!(false))]);
    let snapshot = find_node_snapshot_json(&root, &other, "header").expect("find header");
    let value: Value = serde_json::from_str(&snapshot).expect("parse snapshot");
    assert_eq!(value["p"]["val"], json!("World"));
}

#[test]
fn snapshot_orders_object_keys_recursively_and_preserves_arrays() {
    let root = demo_root();
    let state = demo_state();

    let snapshot = find_node_snapshot_json(&root, &state, "meta").expect("find meta");
    assert_eq!(
        snapshot,
        r#"{"c":null,"for":null,"i":"meta","if":null,"o":null,"on":null,"p":{"cfg":{"a":[3,1,2],"y":true},"z":1},"r":null,"st":null,"t":"Box"}"#
    );
}

#[test]
fn root_snapshot_keeps_child_order_and_subtree() {
    let root = demo_root();
    let state = demo_state();

    let snapshot = find_node_snapshot_json(&root, &state, "root").expect("find root");
    assert_eq!(
        child_ids(&snapshot),
        vec![
            "header".to_string(),
            "meta".to_string(),
            "row:a".to_string(),
            "row:b".to_string(),
        ]
    );

    // The subtree travels with the node.
    let value: Value = serde_json::from_str(&snapshot).expect("parse snapshot");
    assert_eq!(value["c"][2]["c"][0]["i"], json!("row:a:label"));
}

#[test]
fn missing_id_is_an_error() {
    let root = demo_root();
    let state = demo_state();

    let err = find_node_snapshot_json(&root, &state, "nope").expect_err("missing id");
    assert_eq!(err.message, "Node not found: nope");
}

#[test]
fn invalid_id_is_an_error() {
    let root = demo_root();
    let state = demo_state();

    let err = find_node_snapshot_json(&root, &state, "bad id").expect_err("invalid id");
    assert_eq!(err.message, "Invalid id: bad id");

    let err = find_node_snapshot_json(&root, &state, "").expect_err("empty id");
    assert_eq!(err.message, "Invalid id: ");
}

#[test]
fn node_hidden_by_if_is_not_found() {
    let root = demo_root();

    let err = find_node_snapshot_json(&root, &demo_state(), "hidden").expect_err("if=false");
    assert_eq!(err.message, "Node not found: hidden");

    // Flipping the state makes the same id resolvable.
    let shown = state_from(&[("title", json!("Hello")), ("show", json!(true))]);
    let snapshot = find_node_snapshot_json(&root, &shown, "hidden").expect("find hidden");
    let value: Value = serde_json::from_str(&snapshot).expect("parse snapshot");
    assert_eq!(value["i"], json!("hidden"));
}

#[test]
fn finds_generated_loop_and_descendant_ids() {
    let root = demo_root();
    let state = demo_state();

    let snapshot = find_node_snapshot_json(&root, &state, "row:a").expect("find row:a");
    assert_eq!(child_ids(&snapshot), vec!["row:a:label".to_string()]);

    let snapshot =
        find_node_snapshot_json(&root, &state, "row:a:label").expect("find row:a:label");
    let value: Value = serde_json::from_str(&snapshot).expect("parse snapshot");
    assert_eq!(value["p"]["val"], json!("a"));

    let snapshot =
        find_node_snapshot_json(&root, &state, "row:b:label").expect("find row:b:label");
    let value: Value = serde_json::from_str(&snapshot).expect("parse snapshot");
    assert_eq!(value["p"]["val"], json!("b"));

    // Only materialized ids exist: the unexpanded loop id does not.
    let err = find_node_snapshot_json(&root, &state, "row").expect_err("unexpanded loop id");
    assert_eq!(err.message, "Node not found: row");

    // Exact equality only: no prefix or partial matching.
    let err = find_node_snapshot_json(&root, &state, "row:").expect_err("partial id");
    assert_eq!(err.message, "Node not found: row:");
}

#[test]
fn snapshot_is_deterministic_across_calls_and_insertion_order() {
    let root = demo_root();
    let forward = state_from(&[("title", json!("Hello")), ("show", json!(false))]);
    let reversed = state_from(&[("show", json!(false)), ("title", json!("Hello"))]);

    let first = find_node_snapshot_json(&root, &forward, "root").expect("find root");
    let second = find_node_snapshot_json(&root, &forward, "root").expect("find root again");
    assert_eq!(first, second, "repeated lookups must be identical");

    let other = find_node_snapshot_json(&root, &reversed, "root").expect("find root");
    assert_eq!(
        first, other,
        "state insertion order must not change the snapshot"
    );
}

#[test]
fn snapshot_is_detached_and_lookup_does_not_mutate_state() {
    let root = demo_root();
    let state = demo_state();
    let before = state.data.clone();

    let first = find_node_snapshot_json(&root, &state, "header").expect("find header");
    assert_eq!(state.data, before, "a lookup must not mutate state");

    let mut value = find_node_snapshot_value(&root, &state, "header").expect("find header");
    let Value::Object(map) = &mut value else {
        panic!("snapshot must be a JSON object");
    };
    map.insert("i".to_string(), json!("tampered"));
    map.insert("injected".to_string(), json!("x"));

    assert_eq!(state.data, before, "mutating a snapshot must not affect state");
    let again = find_node_snapshot_json(&root, &state, "header").expect("find header again");
    assert_eq!(again, first, "mutating a snapshot must not affect the engine");
}

#[test]
fn materialization_errors_propagate() {
    let state = LaplogosState::default();

    let duplicated = root_from(json!({
        "v": 1,
        "root": {
            "i": "root",
            "t": "Box",
            "c": [
                { "i": "dup", "t": "Txt", "p": { "val": "A" } },
                { "i": "dup", "t": "Txt", "p": { "val": "B" } }
            ]
        }
    }));
    let err = find_node_snapshot_json(&duplicated, &state, "root").expect_err("duplicate id");
    assert_eq!(err.message, "Duplicate id: dup");

    let hidden_root = root_from(json!({
        "v": 1,
        "root": { "i": "root", "t": "Box", "if": "$show" }
    }));
    let mut hidden_state = LaplogosState::default();
    hidden_state.data.insert("show".to_string(), json!(false));
    let err =
        find_node_snapshot_json(&hidden_root, &hidden_state, "root").expect_err("root if=false");
    assert_eq!(err.message, "root if=false is fatal");
}
