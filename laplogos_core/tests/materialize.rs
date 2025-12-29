use laplogos_core::{materialize::materialize_one_root, normalize_page, LaplogosState};
use serde_json::{json, Value};

#[test]
fn materialize_resolves_text_and_expands_for() {
    let raw = json!({
        "v": 1,
        "root": {
            "i": "root",
            "t": "Box",
            "c": [
                { "i": "hello", "t": "Txt", "p": { "val": "$name" } },
                { "i": "row", "t": "Txt", "p": { "val": "$item" }, "for": ["a", "b"] }
            ]
        }
    });

    let page = normalize_page(raw).expect("normalize page");
    let mut state = LaplogosState::default();
    state
        .data
        .insert("name".to_string(), Value::String("Lap".to_string()));

    let mut warnings = Vec::new();
    let root = materialize_one_root(&page.root, &state, &mut warnings).expect("materialize");
    let children = root.c.expect("children");
    assert_eq!(children.len(), 3);

    assert_eq!(children[0].i, "hello");
    assert_eq!(
        children[0].p.get("val"),
        Some(&Value::String("Lap".to_string()))
    );

    assert_eq!(children[1].i, "row:a");
    assert_eq!(
        children[1].p.get("val"),
        Some(&Value::String("a".to_string()))
    );

    assert_eq!(children[2].i, "row:b");
    assert_eq!(
        children[2].p.get("val"),
        Some(&Value::String("b".to_string()))
    );
}
