use laplogos_core::LaplogosState;
use serde_json::{json, Value};

fn state_from(pairs: &[(&str, Value)]) -> LaplogosState {
    let mut state = LaplogosState::default();
    for (key, val) in pairs {
        state.data.insert((*key).to_string(), val.clone());
    }
    state
}

#[test]
fn snapshot_of_empty_state_is_empty_object() {
    assert_eq!(LaplogosState::default().snapshot_json(), "{}");
}

#[test]
fn snapshot_is_deterministic_for_equal_state() {
    let forward = state_from(&[
        ("alpha", json!(1)),
        ("beta", json!("two")),
        ("gamma", json!([1, 2, 3])),
        ("delta", json!(true)),
    ]);
    let reversed = state_from(&[
        ("delta", json!(true)),
        ("gamma", json!([1, 2, 3])),
        ("beta", json!("two")),
        ("alpha", json!(1)),
    ]);

    let first = forward.snapshot_json();
    let second = forward.snapshot_json();
    assert_eq!(first, second, "repeated snapshots must be identical");
    assert_eq!(
        first,
        reversed.snapshot_json(),
        "insertion order must not change the snapshot"
    );
    assert_eq!(
        first,
        r#"{"alpha":1,"beta":"two","delta":true,"gamma":[1,2,3]}"#
    );
}

#[test]
fn snapshot_orders_nested_object_keys() {
    let state = state_from(&[(
        "profile",
        json!({
            "zip": "0001",
            "name": { "last": "B", "first": "A" },
            "tags": [{ "z": 1, "a": 2 }]
        }),
    )]);

    assert_eq!(
        state.snapshot_json(),
        r#"{"profile":{"name":{"first":"A","last":"B"},"tags":[{"a":2,"z":1}],"zip":"0001"}}"#
    );
}

#[test]
fn snapshot_does_not_mutate_state_and_is_detached() {
    let state = state_from(&[("count", json!(1)), ("nested", json!({ "a": 1 }))]);
    let before = state.data.clone();

    let mut snapshot = state.snapshot_value();
    assert_eq!(state.data, before, "taking a snapshot must not mutate state");

    let Value::Object(map) = &mut snapshot else {
        panic!("snapshot must be a JSON object");
    };
    map.insert("count".to_string(), json!(999));
    map.insert("injected".to_string(), json!("x"));

    assert_eq!(
        state.data, before,
        "mutating the snapshot must not affect state"
    );
    assert_eq!(state.data.get("count"), Some(&json!(1)));
    assert!(!state.data.contains_key("injected"));
    assert_eq!(state.snapshot_json(), r#"{"count":1,"nested":{"a":1}}"#);
}

#[test]
fn snapshot_matches_existing_state_serialization() {
    let state = state_from(&[("b", json!(2)), ("a", json!({ "y": 1, "x": 0 }))]);

    // `LaplogosEngine::get_state_json` serializes `state.data` directly; the new
    // snapshot must carry exactly the same data, only canonically ordered.
    let existing_text = serde_json::to_string(&state.data).expect("serialize state data");
    let existing: Value = serde_json::from_str(&existing_text).expect("parse existing state json");
    let snapshot: Value = serde_json::from_str(&state.snapshot_json()).expect("parse snapshot");

    assert_eq!(snapshot, existing);
    assert_eq!(snapshot, json!({ "a": { "x": 0, "y": 1 }, "b": 2 }));
}
