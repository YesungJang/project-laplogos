use laplogos_core::{materialize::materialize_one_root, render_to_string, LaplogosPage, LaplogosState};
use serde_json::Value;

#[test]
fn deterministic_render_snapshot() {
    let page: LaplogosPage = serde_json::from_str(
        r#"{"v":1,"root":{"i":"root","t":"Box","p":{"class":"shell"},"c":[{"i":"t1","t":"Txt","p":{"val":"$count"}}]}}"#,
    )
    .expect("parse json");
    let mut state = LaplogosState::default();
    state.data.insert(
        "count".to_string(),
        Value::Number(serde_json::Number::from(5)),
    );

    let mut warnings = Vec::new();
    let root = materialize_one_root(&page.root, &state, &mut warnings).expect("materialize");
    let html = render_to_string(&root);

    assert_eq!(
        html,
        "<div data-laplogos-id=\"root\" data-laplogos-root=\"1\" class=\"shell\"><p data-laplogos-id=\"t1\">5</p></div>"
    );
}

#[test]
fn render_skips_unsafe_prop_attributes() {
    let page: LaplogosPage = serde_json::from_str(
        r#"{"v":1,"root":{"i":"root","t":"Box","p":{"onclick":"alert(1)","data-test":"ok","bad name":"x","data-laplogos-id":"evil-id"},"c":[{"i":"t1","t":"Txt","p":{"val":"Hi"}}]}}"#,
    )
    .expect("parse json");
    let state = LaplogosState::default();

    let mut warnings = Vec::new();
    let root = materialize_one_root(&page.root, &state, &mut warnings).expect("materialize");
    let html = render_to_string(&root);

    assert!(html.contains("data-test=\"ok\""));
    assert!(!html.contains("onclick"));
    assert!(!html.contains("bad name"));
    assert!(!html.contains("evil-id"));
}

#[test]
fn render_skips_disallowed_href_schemes() {
    let page: LaplogosPage = serde_json::from_str(
        r#"{"v":1,"root":{"i":"root","t":"Box","c":[{"i":"ok","t":"Lnk","p":{"href":"https://example.com","label":"ok"}},{"i":"bad1","t":"Lnk","p":{"href":"//example.com","label":"bad"}},{"i":"bad2","t":"Lnk","p":{"href":"mailto:hello@example.com","label":"bad2"}}]}}"#,
    )
    .expect("parse json");
    let state = LaplogosState::default();

    let mut warnings = Vec::new();
    let root = materialize_one_root(&page.root, &state, &mut warnings).expect("materialize");
    let html = render_to_string(&root);

    assert!(html.contains("href=\"https://example.com\""));
    assert!(!html.contains("href=\"//example.com\""));
    assert!(!html.contains("mailto:"));
}
