use laplogos_core::normalize_page;
use serde_json::{json, Value};

#[test]
fn normalize_strips_packed_fields_in_unpacked_page() {
    let raw = json!({
        "v": 1,
        "root": {
            "i": "root",
            "t": "Box",
            "r": "ignored",
            "st": "style1",
            "o": { "class": "x" },
            "c": [
                {
                    "i": "child",
                    "t": "Txt",
                    "r": "ignored2",
                    "st": "style2",
                    "o": { "val": "hello" },
                    "p": { "val": "hello" }
                }
            ]
        }
    });

    let page = normalize_page(raw).expect("normalize page");
    assert!(page.defs.is_none());
    assert!(page.styles.is_none());
    assert!(page.root.r.is_none());
    assert!(page.root.st.is_none());
    assert!(page.root.o.is_none());

    let child = page.root.c.as_ref().unwrap().first().unwrap();
    assert!(child.r.is_none());
    assert!(child.st.is_none());
    assert!(child.o.is_none());
}

#[test]
fn normalize_packed_applies_ref_style_and_override() {
    let raw = json!({
        "v": 1,
        "defs": {
            "card": { "i": "card", "t": "Txt", "p": { "val": "base" } }
        },
        "styles": {
            "accent": { "class": "accented" }
        },
        "root": {
            "i": "root",
            "t": "Box",
            "c": [
                {
                    "i": "title",
                    "r": "card",
                    "st": "accent",
                    "o": { "val": "override" }
                }
            ]
        }
    });

    let page = normalize_page(raw).expect("normalize page");
    let child = page.root.c.as_ref().unwrap().first().unwrap();

    assert_eq!(child.t, "Txt");
    assert_eq!(
        child.p.get("val"),
        Some(&Value::String("override".to_string()))
    );
    assert_eq!(
        child.p.get("class"),
        Some(&Value::String("accented".to_string()))
    );
    assert!(child.r.is_none());
    assert!(child.st.is_none());
    assert!(child.o.is_none());
}
