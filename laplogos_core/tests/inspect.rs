use laplogos_core::{
    inspect_snapshot_json, inspect_snapshot_value, normalize_page, LaplogosPage, LaplogosState,
};
use serde_json::{json, Value};

fn page_from(raw: Value) -> LaplogosPage {
    normalize_page(raw).expect("normalize page")
}

fn state_from(pairs: &[(&str, Value)]) -> LaplogosState {
    let mut state = LaplogosState::default();
    for (key, val) in pairs {
        state.data.insert((*key).to_string(), val.clone());
    }
    state
}

#[test]
fn snapshot_resolves_state_substitutions() {
    let page = page_from(json!({
        "v": 1,
        "root": {
            "i": "root",
            "t": "Box",
            "c": [
                { "i": "txt", "t": "Txt", "p": { "val": "$name" } },
                { "i": "btn", "t": "Btn", "p": { "label": "$name" } },
                { "i": "lnk", "t": "Lnk", "p": { "href": "/go", "label": "$name" } },
                { "i": "inp", "t": "Inp", "p": { "val": "$name" } }
            ]
        }
    }));
    let state = state_from(&[("name", json!("Lap"))]);

    let snapshot = inspect_snapshot_value(&page.root, &state).expect("inspect");
    assert_eq!(snapshot["i"], json!("root"));

    let children = snapshot["c"].as_array().expect("children");
    assert_eq!(children.len(), 4);
    assert_eq!(children[0]["p"]["val"], json!("Lap"));
    assert_eq!(children[1]["p"]["label"], json!("Lap"));
    assert_eq!(children[2]["p"]["label"], json!("Lap"));
    assert_eq!(children[2]["p"]["href"], json!("/go"));
    assert_eq!(children[3]["p"]["val"], json!("Lap"));

    // `bind_key` stays out of the serialized node representation.
    assert!(children[3].get("bind_key").is_none());
}

#[test]
fn snapshot_applies_conditionals() {
    let page = page_from(json!({
        "v": 1,
        "root": {
            "i": "root",
            "t": "Box",
            "c": [
                { "i": "shown", "t": "Txt", "p": { "val": "yes" }, "if": "$show" },
                { "i": "hidden", "t": "Txt", "p": { "val": "no" }, "if": "$hide" },
                { "i": "missing", "t": "Txt", "p": { "val": "gone" }, "if": "$unknown" }
            ]
        }
    }));
    let state = state_from(&[("show", json!(true)), ("hide", json!(false))]);

    let snapshot = inspect_snapshot_value(&page.root, &state).expect("inspect");
    let children = snapshot["c"].as_array().expect("children");
    assert_eq!(children.len(), 1);
    assert_eq!(children[0]["i"], json!("shown"));
    assert!(children[0]["if"].is_null());
}

#[test]
fn snapshot_treats_non_boolean_if_as_hidden_not_an_error() {
    let page = page_from(json!({
        "v": 1,
        "root": {
            "i": "root",
            "t": "Box",
            "c": [
                { "i": "always", "t": "Txt", "p": { "val": "here" } },
                { "i": "maybe", "t": "Txt", "p": { "val": "nope" }, "if": "$flag" }
            ]
        }
    }));
    let state = state_from(&[("flag", json!("yes"))]);

    let snapshot = inspect_snapshot_value(&page.root, &state).expect("non-boolean if is a warning");
    let children = snapshot["c"].as_array().expect("children");
    assert_eq!(children.len(), 1);
    assert_eq!(children[0]["i"], json!("always"));
}

#[test]
fn snapshot_expands_loops_with_materialized_ids_in_order() {
    let page = page_from(json!({
        "v": 1,
        "root": {
            "i": "root",
            "t": "Box",
            "c": [
                {
                    "i": "row",
                    "t": "Box",
                    "for": ["a", "b"],
                    "c": [
                        { "i": "label", "t": "Txt", "p": { "val": "$item" } },
                        { "i": "idx", "t": "Txt", "p": { "val": "$key" } }
                    ]
                }
            ]
        }
    }));
    let state = LaplogosState::default();

    let snapshot = inspect_snapshot_value(&page.root, &state).expect("inspect");
    let rows = snapshot["c"].as_array().expect("rows");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["i"], json!("row:a"));
    assert_eq!(rows[1]["i"], json!("row:b"));
    assert!(rows[0]["for"].is_null());

    let first = rows[0]["c"].as_array().expect("row children");
    assert_eq!(first[0]["i"], json!("row:a:label"));
    assert_eq!(first[0]["p"]["val"], json!("a"));
    assert_eq!(first[1]["i"], json!("row:a:idx"));
    assert_eq!(first[1]["p"]["val"], json!("0"));

    let second = rows[1]["c"].as_array().expect("row children");
    assert_eq!(second[0]["i"], json!("row:b:label"));
    assert_eq!(second[0]["p"]["val"], json!("b"));
    assert_eq!(second[1]["i"], json!("row:b:idx"));
    assert_eq!(second[1]["p"]["val"], json!("1"));
}

#[test]
fn snapshot_uses_canonical_key_order_and_existing_node_shape() {
    let page = page_from(json!({
        "v": 1,
        "root": { "i": "root", "t": "Txt", "p": { "zz": "1", "aa": "2", "val": "$count" } }
    }));
    let state = state_from(&[("count", json!(5))]);

    assert_eq!(
        inspect_snapshot_json(&page.root, &state).expect("inspect"),
        r#"{"c":null,"for":null,"i":"root","if":null,"o":null,"on":null,"p":{"aa":"2","val":"5","zz":"1"},"r":null,"st":null,"t":"Txt"}"#
    );
}

#[test]
fn snapshot_is_deterministic_for_equal_state() {
    let page = page_from(json!({
        "v": 1,
        "root": {
            "i": "root",
            "t": "Box",
            "c": [
                { "i": "a", "t": "Txt", "p": { "val": "$alpha" } },
                { "i": "b", "t": "Txt", "p": { "val": "$beta" } }
            ]
        }
    }));

    let forward = state_from(&[
        ("alpha", json!("one")),
        ("beta", json!("two")),
        ("gamma", json!({ "z": 1, "a": 2 })),
        ("delta", json!(true)),
    ]);
    let reversed = state_from(&[
        ("delta", json!(true)),
        ("gamma", json!({ "z": 1, "a": 2 })),
        ("beta", json!("two")),
        ("alpha", json!("one")),
    ]);

    let first = inspect_snapshot_json(&page.root, &forward).expect("inspect");
    let second = inspect_snapshot_json(&page.root, &forward).expect("inspect");
    assert_eq!(first, second, "repeated snapshots must be identical");
    assert_eq!(
        first,
        inspect_snapshot_json(&page.root, &reversed).expect("inspect"),
        "state insertion order must not change the snapshot"
    );
}

#[test]
fn snapshot_sorts_nested_object_keys_and_keeps_order() {
    let page = page_from(json!({
        "v": 1,
        "root": {
            "i": "root",
            "t": "Box",
            "p": { "data": { "z": 1, "a": { "y": 2, "b": 3 } }, "list": [{ "z": 1, "a": 2 }] },
            "c": [
                { "i": "first", "t": "Txt", "p": { "val": "1" } },
                { "i": "second", "t": "Txt", "p": { "val": "2" } }
            ]
        }
    }));
    let state = LaplogosState::default();

    let json_text = inspect_snapshot_json(&page.root, &state).expect("inspect");
    assert!(json_text.contains(r#""data":{"a":{"b":3,"y":2},"z":1}"#));
    assert!(json_text.contains(r#""list":[{"a":2,"z":1}]"#));

    let snapshot = inspect_snapshot_value(&page.root, &state).expect("inspect");
    let children = snapshot["c"].as_array().expect("children");
    assert_eq!(children[0]["i"], json!("first"));
    assert_eq!(children[1]["i"], json!("second"));
}

#[test]
fn snapshot_is_detached_and_does_not_mutate_inputs() {
    let page = page_from(json!({
        "v": 1,
        "root": {
            "i": "root",
            "t": "Box",
            "c": [{ "i": "txt", "t": "Txt", "p": { "val": "$name" } }]
        }
    }));
    let state = state_from(&[("name", json!("Lap")), ("nested", json!({ "a": 1 }))]);

    let state_before = state.data.clone();
    let page_before = serde_json::to_value(&page).expect("serialize page");

    let mut snapshot = inspect_snapshot_value(&page.root, &state).expect("inspect");
    let Value::Object(map) = &mut snapshot else {
        panic!("snapshot must be a JSON object");
    };
    map.insert("i".to_string(), json!("hacked"));
    map.insert("injected".to_string(), json!("x"));

    assert_eq!(state.data, state_before, "state must not be mutated");
    assert_eq!(
        serde_json::to_value(&page).expect("serialize page"),
        page_before,
        "page must not be mutated"
    );

    let again = inspect_snapshot_json(&page.root, &state).expect("inspect");
    assert!(!again.contains("hacked"));
    assert!(!again.contains("injected"));
}

#[test]
fn snapshot_does_not_require_a_prior_render() {
    // The engine caches a materialized tree only once it has rendered; the
    // snapshot path materializes the normalized page directly, so a freshly
    // normalized page works with no render or hydrate step.
    let page = page_from(json!({
        "v": 1,
        "root": { "i": "root", "t": "Txt", "p": { "val": "Hi" } }
    }));
    let state = LaplogosState::default();

    let snapshot = inspect_snapshot_value(&page.root, &state).expect("inspect");
    assert_eq!(snapshot["i"], json!("root"));
    assert_eq!(snapshot["p"]["val"], json!("Hi"));
}

#[test]
fn snapshot_surfaces_root_if_false_error() {
    let page = page_from(json!({
        "v": 1,
        "root": { "i": "root", "t": "Box", "if": "$show" }
    }));
    let state = state_from(&[("show", json!(false))]);

    let err = inspect_snapshot_json(&page.root, &state).expect_err("root if=false is fatal");
    assert!(err.message.contains("root if=false"));
}

#[test]
fn snapshot_surfaces_duplicate_and_invalid_id_errors() {
    let duplicate = page_from(json!({
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
    let state = LaplogosState::default();
    let err = inspect_snapshot_value(&duplicate.root, &state).expect_err("duplicate id");
    assert!(err.message.contains("Duplicate id"));

    let invalid = page_from(json!({ "v": 1, "root": { "i": "bad id", "t": "Box" } }));
    let err = inspect_snapshot_value(&invalid.root, &state).expect_err("invalid id");
    assert!(err.message.contains("Invalid id"));
}

#[test]
fn snapshot_surfaces_loop_limit_error() {
    let items: Vec<Value> = (0..201).map(|i| json!(format!("item{}", i))).collect();
    let page = page_from(json!({
        "v": 1,
        "root": {
            "i": "root",
            "t": "Box",
            "c": [{ "i": "row", "t": "Txt", "for": items, "p": { "val": "$item" } }]
        }
    }));
    let state = LaplogosState::default();

    let err = inspect_snapshot_json(&page.root, &state).expect_err("loop limit exceeded");
    assert!(err.message.contains("loop item limit exceeded"));
}
