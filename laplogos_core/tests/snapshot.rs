use laplogos_core::{materialize::materialize_one_root, render_to_string, LaplogosPage, LaplogosState};
use serde_json::Value;

#[test]
fn deterministic_render_snapshot() {
    let page: LaplogosPage = serde_json::from_str(
        r#"{"v":1,"root":{"i":"root","t":"Box","c":[{"i":"t1","t":"Txt","p":{"val":"$count"}}]}}"#,
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
        "<div id=\"root\" data-laplogos-id=\"root\"><p id=\"t1\" data-laplogos-id=\"t1\">5</p></div>"
    );
}
