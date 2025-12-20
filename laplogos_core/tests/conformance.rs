use std::fs;
use std::path::Path;

use laplogos_core::{materialize::materialize_one_root, LaplogosPage, LaplogosState};
use serde_json::Value;

fn load_page(path: &Path) -> LaplogosPage {
    let text = fs::read_to_string(path).expect("read json");
    serde_json::from_str(&text).expect("parse page json")
}

fn state_for(path: &Path) -> LaplogosState {
    let mut state = LaplogosState::default();
    let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    if name.contains("if_non_bool") {
        state
            .data
            .insert("flag".to_string(), Value::String("yes".to_string()));
    }
    state
}

#[test]
fn valid_specs_pass() {
    let dir = Path::new("tests/data");
    for entry in fs::read_dir(dir).expect("read tests/data") {
        let entry = entry.expect("read entry");
        let path = entry.path();
        if path
            .file_name()
            .and_then(|s| s.to_str())
            .map(|n| n.starts_with("valid_"))
            != Some(true)
        {
            continue;
        }
        let page = load_page(&path);
        let mut warnings = Vec::new();
        let state = state_for(&path);
        let result = materialize_one_root(&page.root, &state, &mut warnings);
        assert!(
            result.is_ok(),
            "valid file failed: {:?}",
            path.file_name()
        );
    }
}

#[test]
fn invalid_specs_fail() {
    let dir = Path::new("tests/data");
    for entry in fs::read_dir(dir).expect("read tests/data") {
        let entry = entry.expect("read entry");
        let path = entry.path();
        if path
            .file_name()
            .and_then(|s| s.to_str())
            .map(|n| n.starts_with("invalid_"))
            != Some(true)
        {
            continue;
        }
        let page = load_page(&path);
        let mut warnings = Vec::new();
        let state = state_for(&path);
        let result = materialize_one_root(&page.root, &state, &mut warnings);
        assert!(
            result.is_err(),
            "invalid file unexpectedly passed: {:?}",
            path.file_name()
        );
    }
}
