use std::collections::HashMap;

use crate::LaplogosNode;

fn is_disallowed_prop_name(key: &str) -> bool {
    let lower = key.trim().to_ascii_lowercase();
    if lower == "data-laplogos-id" || lower == "data-laplogos-root" {
        return true;
    }
    lower.starts_with("on")
}

fn is_safe_prop_name(key: &str) -> bool {
    if key.is_empty() {
        return false;
    }
    let mut chars = key.chars();
    let first = match chars.next() {
        Some(c) => c,
        None => return false,
    };
    if !(first.is_ascii_alphabetic() || first == '_' || first == ':') {
        return false;
    }
    for c in chars {
        if !(c.is_ascii_alphanumeric() || c == '_' || c == ':' || c == '-' || c == '.') {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone)]
pub enum Patch {
    Append { parent_id: String, node: LaplogosNode },
    Remove { id: String },
    Replace { id: String, node: LaplogosNode },
    UpdateText { id: String, val: String },
    UpdateAttr { id: String, key: String, val: String },
    RemoveAttr { id: String, key: String },
    ReorderChildren { parent_id: String, ordered_ids: Vec<String> },
}

pub fn diff(old: &LaplogosNode, new: &LaplogosNode) -> Vec<Patch> {
    let mut patches = Vec::new();

    if old.i != new.i || old.t != new.t {
        patches.push(Patch::Replace {
            id: old.i.clone(),
            node: new.clone(),
        });
        return patches;
    }

    if old.on != new.on {
        patches.push(Patch::Replace {
            id: old.i.clone(),
            node: new.clone(),
        });
        return patches;
    }

    if old.t == "Lnk" {
        let old_href = old.p.get("href");
        let new_href = new.p.get("href");
        if old_href != new_href {
            patches.push(Patch::Replace {
                id: old.i.clone(),
                node: new.clone(),
            });
            return patches;
        }
    }

    if old.t == "Inp" && old.bind_key != new.bind_key {
        patches.push(Patch::Replace {
            id: old.i.clone(),
            node: new.clone(),
        });
        return patches;
    }

    diff_props(old, new, &mut patches);
    diff_children(old, new, &mut patches);

    patches
}

fn diff_props(old: &LaplogosNode, new: &LaplogosNode, patches: &mut Vec<Patch>) {
    let mut new_keys: Vec<&String> = new.p.keys().collect();
    new_keys.sort();
    for key in new_keys {
        if is_disallowed_prop_name(key) || !is_safe_prop_name(key) {
            continue;
        }
        let new_val = new.p.get(key).expect("key from map");
        let old_val = old.p.get(key);
        if old_val == Some(new_val) {
            continue;
        }

        match (new.t.as_str(), key.as_str()) {
            ("Txt", "val") => patches.push(Patch::UpdateText {
                id: new.i.clone(),
                val: value_to_string(new_val),
            }),
            ("Btn", "label") => patches.push(Patch::UpdateText {
                id: new.i.clone(),
                val: value_to_string(new_val),
            }),
            ("Lnk", "label") => patches.push(Patch::UpdateText {
                id: new.i.clone(),
                val: value_to_string(new_val),
            }),
            ("Inp", "val") => patches.push(Patch::UpdateAttr {
                id: new.i.clone(),
                key: "value".to_string(),
                val: value_to_string(new_val),
            }),
            _ => patches.push(Patch::UpdateAttr {
                id: new.i.clone(),
                key: key.clone(),
                val: value_to_string(new_val),
            }),
        }
    }

    let mut old_keys: Vec<&String> = old.p.keys().collect();
    old_keys.sort();
    for key in old_keys {
        if is_disallowed_prop_name(key) || !is_safe_prop_name(key) {
            continue;
        }
        if new.p.contains_key(key) {
            continue;
        }

        match (new.t.as_str(), key.as_str()) {
            ("Txt", "val") => patches.push(Patch::UpdateText {
                id: new.i.clone(),
                val: "".to_string(),
            }),
            ("Btn", "label") => patches.push(Patch::UpdateText {
                id: new.i.clone(),
                val: "".to_string(),
            }),
            ("Lnk", "label") => patches.push(Patch::UpdateText {
                id: new.i.clone(),
                val: "".to_string(),
            }),
            ("Inp", "val") => patches.push(Patch::UpdateAttr {
                id: new.i.clone(),
                key: "value".to_string(),
                val: "".to_string(),
            }),
            _ => patches.push(Patch::RemoveAttr {
                id: new.i.clone(),
                key: key.clone(),
            }),
        }
    }
}

fn diff_children(old: &LaplogosNode, new: &LaplogosNode, patches: &mut Vec<Patch>) {
    let old_children = old.c.as_ref().cloned().unwrap_or_default();
    let new_children = new.c.as_ref().cloned().unwrap_or_default();

    let old_map: HashMap<String, LaplogosNode> =
        old_children.iter().cloned().map(|n| (n.i.clone(), n)).collect();
    let new_map: HashMap<String, LaplogosNode> =
        new_children.iter().cloned().map(|n| (n.i.clone(), n)).collect();

    for old_child in &old_children {
        if !new_map.contains_key(&old_child.i) {
            patches.push(Patch::Remove {
                id: old_child.i.clone(),
            });
        } else {
            let new_child = new_map.get(&old_child.i).expect("child exists");
            patches.extend(diff(old_child, new_child));
        }
    }

    for new_child in &new_children {
        if !old_map.contains_key(&new_child.i) {
            patches.push(Patch::Append {
                parent_id: new.i.clone(),
                node: new_child.clone(),
            });
        }
    }

    let old_ids: Vec<String> = old_children.iter().map(|n| n.i.clone()).collect();
    let new_ids: Vec<String> = new_children.iter().map(|n| n.i.clone()).collect();

    if !new_ids.is_empty() && old_ids != new_ids {
        patches.push(Patch::ReorderChildren {
            parent_id: new.i.clone(),
            ordered_ids: new_ids,
        });
    }
}

fn value_to_string(val: &serde_json::Value) -> String {
    val.as_str()
        .map(|s| s.to_string())
        .unwrap_or_else(|| val.to_string())
}
