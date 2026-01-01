use serde_json::Value;
use std::collections::HashSet;

use crate::{store::LaplogosState, LaplogosNode};

const MAX_NODES: usize = 5000;
const MAX_DEPTH: usize = 256;
const MAX_LOOP_ITEMS: usize = 200;

#[derive(Debug, Clone, Default)]
pub struct MaterializeScope {
    pub item: Option<String>,
    pub key: Option<String>,
    pub id_prefix: Option<String>,
}

#[derive(Debug, Default)]
struct MaterializeContext {
    node_count: usize,
    ids: HashSet<String>,
}

impl MaterializeContext {
    fn bump(&mut self) -> Result<(), LaplogosError> {
        self.node_count += 1;
        if self.node_count > MAX_NODES {
            return Err(LaplogosError::new(format!(
                "max nodes exceeded: {} > {}",
                self.node_count, MAX_NODES
            )));
        }
        Ok(())
    }

    fn register_id(&mut self, id: &str) -> Result<(), LaplogosError> {
        if !self.ids.insert(id.to_string()) {
            return Err(LaplogosError::new(format!("Duplicate id: {}", id)));
        }
        Ok(())
    }
}

pub fn resolve_text(val: &Value, state: &LaplogosState, scope: &MaterializeScope) -> String {
    let raw = match val.as_str() {
        Some(s) => s,
        None => return val.to_string(),
    };

    if raw == "$item" {
        if let Some(item) = scope.item.as_ref() {
            return item.clone();
        }
    }

    if raw == "$key" {
        if let Some(key) = scope.key.as_ref() {
            return key.clone();
        }
    }

    if let Some(stripped) = raw.strip_prefix('$') {
        if let Some(value) = state.data.get(stripped) {
            if let Some(text) = value.as_str() {
                return text.to_string();
            }
            return value.to_string();
        }
    }

    raw.to_string()
}

pub fn resolve_if(val: &Value, state: &LaplogosState) -> (bool, Option<String>) {
    let key = match val.as_str() {
        Some(s) => s,
        None => return (false, Some("if must be a string key".to_string())),
    };

    let key = key.strip_prefix('$').unwrap_or(key);
    match state.data.get(key) {
        Some(Value::Bool(true)) => (true, None),
        Some(Value::Bool(false)) => (false, None),
        Some(_) => (false, Some(format!("if key '{}' is not boolean", key))),
        None => (false, None),
    }
}

#[derive(Debug, Clone)]
pub struct LaplogosError {
    pub message: String,
}

impl LaplogosError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

pub fn is_valid_id(id: &str) -> bool {
    if id.is_empty() {
        return false;
    }
    id.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ':' || c == '-')
}

pub fn validate_global_ids(node: &LaplogosNode) -> Result<(), String> {
    if !is_valid_id(&node.i) {
        return Err(format!("Invalid id: {}", node.i));
    }
    if let Some(children) = node.c.as_ref() {
        for child in children {
            validate_global_ids(child)?;
        }
    }
    Ok(())
}

fn materialize_many(
    node: &LaplogosNode,
    state: &LaplogosState,
    scope: &MaterializeScope,
    warnings: &mut Vec<String>,
    ctx: &mut MaterializeContext,
    depth: usize,
) -> Result<Vec<LaplogosNode>, LaplogosError> {
    if depth > MAX_DEPTH {
        return Err(LaplogosError::new(format!(
            "max depth exceeded: {} > {}",
            depth, MAX_DEPTH
        )));
    }

    let base_id = match scope.id_prefix.as_ref() {
        Some(prefix) => format!("{}:{}", prefix, node.i),
        None => node.i.clone(),
    };

    if !is_valid_id(&base_id) {
        return Err(LaplogosError::new(format!("Invalid id: {}", base_id)));
    }

    if let Some(if_val) = node.if_.as_ref() {
        let (allowed, warn) = resolve_if(if_val, state);
        if let Some(warn) = warn {
            warnings.push(warn);
        }
        if !allowed {
            return Ok(Vec::new());
        }
    }

    if let Some(for_val) = node.for_.as_ref() {
        let items = match for_val {
            Value::Array(items) => items,
            _ => return Err(LaplogosError::new("for must be an array")),
        };
        if items.len() > MAX_LOOP_ITEMS {
            return Err(LaplogosError::new(format!(
                "loop item limit exceeded: {} > {}",
                items.len(),
                MAX_LOOP_ITEMS
            )));
        }

        let mut output = Vec::new();
        let mut seen = HashSet::new();
        let mut kind: Option<&'static str> = None;

        for (idx, item) in items.iter().enumerate() {
            let item_str = match item {
                Value::String(s) => {
                    kind.get_or_insert("str");
                    if kind != Some("str") {
                        return Err(LaplogosError::new("for array must be all strings or all numbers"));
                    }
                    s.clone()
                }
                Value::Number(n) => {
                    kind.get_or_insert("num");
                    if kind != Some("num") {
                        return Err(LaplogosError::new("for array must be all strings or all numbers"));
                    }
                    n.to_string()
                }
                _ => return Err(LaplogosError::new("for array items must be string or number")),
            };

            if !is_valid_id(&item_str) {
                return Err(LaplogosError::new(format!("Invalid loop item id: {}", item_str)));
            }
            if !seen.insert(item_str.clone()) {
                return Err(LaplogosError::new(format!("Duplicate loop item: {}", item_str)));
            }

            let mut child_scope = scope.clone();
            child_scope.item = Some(item_str.clone());
            child_scope.key = Some(idx.to_string());

            ctx.bump()?;

            let mut node_copy = node.clone();
            node_copy.for_ = None;
            node_copy.if_ = None;
            node_copy.i = format!("{}:{}", base_id, item_str);

            if !is_valid_id(&node_copy.i) {
                return Err(LaplogosError::new(format!("Invalid generated id: {}", node_copy.i)));
            }

            ctx.register_id(&node_copy.i)?;
            child_scope.id_prefix = Some(node_copy.i.clone());
            apply_text(&mut node_copy, state, &child_scope);
            materialize_children(&mut node_copy, state, &child_scope, warnings, ctx, depth)?;
            output.push(node_copy);
        }

        return Ok(output);
    }

    ctx.bump()?;
    let mut node_copy = node.clone();
    node_copy.for_ = None;
    node_copy.if_ = None;
    node_copy.i = base_id;
    ctx.register_id(&node_copy.i)?;
    apply_text(&mut node_copy, state, scope);
    materialize_children(&mut node_copy, state, scope, warnings, ctx, depth)?;
    Ok(vec![node_copy])
}

pub fn materialize_one_root(
    root: &LaplogosNode,
    state: &LaplogosState,
    warnings: &mut Vec<String>,
) -> Result<LaplogosNode, LaplogosError> {
    if root.for_.is_some() {
        return Err(LaplogosError::new("root cannot have for"));
    }

    check_depth_limit(root)?;

    let mut ctx = MaterializeContext::default();
    let nodes = materialize_many(
        root,
        state,
        &MaterializeScope::default(),
        warnings,
        &mut ctx,
        1,
    )?;
    match nodes.len() {
        1 => Ok(nodes.into_iter().next().unwrap()),
        0 => Err(LaplogosError::new("root if=false is fatal")),
        _ => Err(LaplogosError::new("root must materialize to a single node")),
    }
}

fn check_depth_limit(root: &LaplogosNode) -> Result<(), LaplogosError> {
    let mut stack = vec![(root, 1usize)];
    while let Some((node, depth)) = stack.pop() {
        if depth > MAX_DEPTH {
            return Err(LaplogosError::new(format!(
                "max depth exceeded: {} > {}",
                depth, MAX_DEPTH
            )));
        }
        if let Some(children) = node.c.as_ref() {
            for child in children {
                stack.push((child, depth + 1));
            }
        }
    }
    Ok(())
}

fn apply_text(node: &mut LaplogosNode, state: &LaplogosState, scope: &MaterializeScope) {
    match node.t.as_str() {
        "Txt" => {
            if let Some(val) = node.p.get("val") {
                let resolved = resolve_text(val, state, scope);
                node.p
                    .insert("val".to_string(), Value::String(resolved));
            }
        }
        "Btn" => {
            if let Some(label) = node.p.get("label") {
                let resolved = resolve_text(label, state, scope);
                node.p
                    .insert("label".to_string(), Value::String(resolved));
            }
        }
        "Lnk" => {
            if let Some(label) = node.p.get("label") {
                let resolved = resolve_text(label, state, scope);
                node.p
                    .insert("label".to_string(), Value::String(resolved));
            }
        }
        "Inp" => {
            if let Some(val) = node.p.get("val") {
                if let Some(raw) = val.as_str() {
                    let mut bind_key = None;
                    if raw == "$item" {
                        if let Some(item) = scope.item.as_ref() {
                            bind_key = Some(item.clone());
                        }
                    }
                    if bind_key.is_none() && raw == "$key" {
                        if let Some(key) = scope.key.as_ref() {
                            bind_key = Some(key.clone());
                        }
                    }
                    if bind_key.is_none() && raw.starts_with('$') && raw.len() > 1 {
                        bind_key = Some(raw.trim_start_matches('$').to_string());
                    }
                    if let Some(key) = bind_key {
                        node.bind_key = Some(key.clone());
                        let resolved = state
                            .data
                            .get(&key)
                            .map(|v| v.as_str().map(|s| s.to_string()).unwrap_or_else(|| v.to_string()))
                            .unwrap_or_default();
                        node.p
                            .insert("val".to_string(), Value::String(resolved));
                        return;
                    }
                }
                let resolved = resolve_text(val, state, scope);
                node.p
                    .insert("val".to_string(), Value::String(resolved));
            }
        }
        _ => {}
    }
}

fn materialize_children(
    node: &mut LaplogosNode,
    state: &LaplogosState,
    scope: &MaterializeScope,
    warnings: &mut Vec<String>,
    ctx: &mut MaterializeContext,
    depth: usize,
) -> Result<(), LaplogosError> {
    let children = match node.c.take() {
        Some(children) => children,
        None => return Ok(()),
    };

    let mut out = Vec::new();
    for child in children {
        let mut materialized = materialize_many(&child, state, scope, warnings, ctx, depth + 1)?;
        out.append(&mut materialized);
    }

    node.c = if out.is_empty() { None } else { Some(out) };
    Ok(())
}
