//! Non-tree output formats: JSON, Markdown, flat list.

use serde_json::{json, Value};

use crate::tree::{Node, NodeKind, human_size};

// ── JSON ─────────────────────────────────────────────────────────────────────

pub fn to_json(node: &Node, include_size: bool) -> Value {
    let mut obj = serde_json::Map::new();
    obj.insert("name".into(), json!(node.name()));
    obj.insert(
        "type".into(),
        json!(match node.kind {
            NodeKind::Dir => "directory",
            NodeKind::File => "file",
            NodeKind::Symlink => "symlink",
        }),
    );
    if include_size {
        obj.insert("size".into(), json!(node.size));
    }
    if node.kind == NodeKind::Dir && !node.children.is_empty() {
        let children: Vec<Value> = node
            .children
            .iter()
            .map(|c| to_json(c, include_size))
            .collect();
        obj.insert("children".into(), json!(children));
    }
    Value::Object(obj)
}

// ── Markdown ──────────────────────────────────────────────────────────────────

pub fn to_markdown(node: &Node, depth: usize) -> String {
    let mut out = String::new();
    let indent = "  ".repeat(depth);
    let marker = if node.kind == NodeKind::Dir { "**" } else { "" };
    out.push_str(&format!(
        "{}- {}{}{}\n",
        indent,
        marker,
        node.name(),
        marker
    ));
    for child in &node.children {
        out.push_str(&to_markdown(child, depth + 1));
    }
    out
}

// ── Flat list ─────────────────────────────────────────────────────────────────

pub fn to_flat_list(node: &Node, base: &std::path::Path) -> Vec<String> {
    let mut out = Vec::new();
    flat_list_inner(node, base, &mut out);
    out
}

fn flat_list_inner(node: &Node, base: &std::path::Path, out: &mut Vec<String>) {
    let rel = node
        .path
        .strip_prefix(base)
        .unwrap_or(&node.path)
        .to_string_lossy()
        .to_string();
    if !rel.is_empty() {
        out.push(rel);
    }
    for child in &node.children {
        flat_list_inner(child, base, out);
    }
}

// ── Summary ───────────────────────────────────────────────────────────────────

pub struct Summary {
    pub dirs: usize,
    pub files: usize,
    pub total_size: u64,
}

impl Summary {
    pub fn from_node(root: &Node) -> Self {
        Self {
            dirs: root.dir_count(),
            files: root.file_count(),
            total_size: root.total_size(),
        }
    }

    pub fn format(&self) -> String {
        format!(
            "\n{} directories, {} files, {}",
            self.dirs,
            self.files,
            human_size(self.total_size)
        )
    }
}
