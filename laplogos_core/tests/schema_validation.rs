use laplogos_core::{materialize::materialize_one_root, normalize_page, LaplogosState};
use serde_json::{json, Value};

#[test]
fn materialize_rejects_invalid_id() {
    let raw = json!({
        "v": 1,
        "root": { "i": "bad id", "t": "Box" }
    });
    let page = normalize_page(raw).expect("normalize page");
    let mut warnings = Vec::new();
    let state = LaplogosState::default();
    let result = materialize_one_root(&page.root, &state, &mut warnings);
    assert!(result.is_err());
}

#[test]
fn root_if_false_is_fatal() {
    let raw = json!({
        "v": 1,
        "root": { "i": "root", "t": "Box", "if": "$show" }
    });
    let page = normalize_page(raw).expect("normalize page");
    let mut state = LaplogosState::default();
    state.data.insert("show".to_string(), Value::Bool(false));
    let mut warnings = Vec::new();
    let result = materialize_one_root(&page.root, &state, &mut warnings);
    assert!(result.is_err());
}

#[test]
fn loop_duplicate_items_fail() {
    let raw = json!({
        "v": 1,
        "root": {
            "i": "root",
            "t": "Box",
            "c": [
                { "i": "row", "t": "Txt", "p": { "val": "$item" }, "for": ["a", "a"] }
            ]
        }
    });
    let page = normalize_page(raw).expect("normalize page");
    let mut warnings = Vec::new();
    let state = LaplogosState::default();
    let result = materialize_one_root(&page.root, &state, &mut warnings);
    assert!(result.is_err());
}
