use std::collections::{HashMap, HashSet};

use crate::AixNode;

#[derive(Debug, Clone)]
pub enum Patch {
    Append { parent_id: String, node: AixNode },
    Remove { id: String },
    Replace { id: String, node: AixNode },
    UpdateText { id: String, val: String },
    UpdateAttr { id: String, key: String, val: String },
    ReorderChildren { parent_id: String, ordered_ids: Vec<String> },
}

pub fn diff(old: &AixNode, new: &AixNode) -> Vec<Patch> {
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

    diff_props(old, new, &mut patches);
    diff_children(old, new, &mut patches);

    patches
}

fn diff_props(old: &AixNode, new: &AixNode, patches: &mut Vec<Patch>) {
    for (key, new_val) in &new.p {
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
}

fn diff_children(old: &AixNode, new: &AixNode, patches: &mut Vec<Patch>) {
    let old_children = old.c.as_ref().cloned().unwrap_or_default();
    let new_children = new.c.as_ref().cloned().unwrap_or_default();

    let old_map: HashMap<String, AixNode> =
        old_children.iter().cloned().map(|n| (n.i.clone(), n)).collect();
    let new_map: HashMap<String, AixNode> =
        new_children.iter().cloned().map(|n| (n.i.clone(), n)).collect();

    for (id, old_child) in &old_map {
        if !new_map.contains_key(id) {
            patches.push(Patch::Remove { id: id.clone() });
        } else {
            let new_child = new_map.get(id).unwrap();
            patches.extend(diff(old_child, new_child));
        }
    }

    for (id, new_child) in &new_map {
        if !old_map.contains_key(id) {
            patches.push(Patch::Append {
                parent_id: new.i.clone(),
                node: new_child.clone(),
            });
        }
    }

    let old_ids: Vec<String> = old_children.iter().map(|n| n.i.clone()).collect();
    let new_ids: Vec<String> = new_children.iter().map(|n| n.i.clone()).collect();

    if old_ids.len() == new_ids.len() {
        let old_set: HashSet<String> = old_ids.iter().cloned().collect();
        let new_set: HashSet<String> = new_ids.iter().cloned().collect();
        if old_set == new_set && old_ids != new_ids {
            patches.push(Patch::ReorderChildren {
                parent_id: new.i.clone(),
                ordered_ids: new_ids,
            });
        }
    }
}

fn value_to_string(val: &serde_json::Value) -> String {
    val.as_str()
        .map(|s| s.to_string())
        .unwrap_or_else(|| val.to_string())
}
