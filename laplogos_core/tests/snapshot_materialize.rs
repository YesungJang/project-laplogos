use laplogos_core::{materialize::materialize_one_root, normalize_page, LaplogosNode, LaplogosState};
use serde_json::{json, Value};
use std::collections::HashMap;

fn node(
    id: &str,
    t: &str,
    p: HashMap<String, Value>,
    c: Option<Vec<LaplogosNode>>,
) -> LaplogosNode {
    LaplogosNode {
        i: id.to_string(),
        t: t.to_string(),
        p,
        c,
        if_: None,
        for_: None,
        on: None,
        r: None,
        st: None,
        o: None,
    }
}

fn props(pairs: &[(&str, Value)]) -> HashMap<String, Value> {
    let mut map = HashMap::new();
    for (key, val) in pairs {
        map.insert((*key).to_string(), val.clone());
    }
    map
}

#[test]
fn snapshot_materialize_with_if_for_ref_and_key() {
    let raw = json!({
        "v": 1,
        "defs": {
            "row": {
                "i": "row",
                "t": "Box",
                "c": [
                    { "i": "label", "t": "Txt", "p": { "val": "$item" } },
                    { "i": "idx", "t": "Txt", "p": { "val": "$key" } },
                    { "i": "title", "t": "Txt", "p": { "val": "$title" } }
                ]
            }
        },
        "root": {
            "i": "root",
            "t": "Box",
            "c": [
                { "i": "header", "t": "Txt", "p": { "val": "$title" }, "if": "$show" },
                { "i": "row", "r": "row", "for": ["a", "b"] }
            ]
        }
    });

    let page = normalize_page(raw).expect("normalize page");
    let mut state = LaplogosState::default();
    state
        .data
        .insert("title".to_string(), Value::String("Hello".to_string()));
    state.data.insert("show".to_string(), Value::Bool(true));

    let mut warnings = Vec::new();
    let root = materialize_one_root(&page.root, &state, &mut warnings).expect("materialize");

    let row_a = node(
        "row:a",
        "Box",
        HashMap::new(),
        Some(vec![
            node("label", "Txt", props(&[("val", Value::String("a".to_string()))]), None),
            node("idx", "Txt", props(&[("val", Value::String("0".to_string()))]), None),
            node(
                "title",
                "Txt",
                props(&[("val", Value::String("Hello".to_string()))]),
                None,
            ),
        ]),
    );
    let row_b = node(
        "row:b",
        "Box",
        HashMap::new(),
        Some(vec![
            node("label", "Txt", props(&[("val", Value::String("b".to_string()))]), None),
            node("idx", "Txt", props(&[("val", Value::String("1".to_string()))]), None),
            node(
                "title",
                "Txt",
                props(&[("val", Value::String("Hello".to_string()))]),
                None,
            ),
        ]),
    );

    let expected = node(
        "root",
        "Box",
        HashMap::new(),
        Some(vec![
            node(
                "header",
                "Txt",
                props(&[("val", Value::String("Hello".to_string()))]),
                None,
            ),
            row_a,
            row_b,
        ]),
    );

    let actual_value = serde_json::to_value(&root).expect("serialize actual");
    let expected_value = serde_json::to_value(&expected).expect("serialize expected");
    assert_eq!(actual_value, expected_value);
}
