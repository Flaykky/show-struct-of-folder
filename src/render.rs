//! Tree-format renderer: walks the in-memory Node tree and produces a String.

use crate::analyze::{self, CodeStats};
use crate::style::{Palette, StyleConfig, git_status_glyph, icon_str, paint_connector,
                   paint_meta, paint_name};
use crate::tree::{Node, NodeKind, human_size};

pub struct RenderOptions {
    pub show_lines: bool,
    pub show_sizes: bool,
    pub show_git: bool,
    pub show_code: bool,
    pub analyze: bool,
    pub full_path: bool,
    pub root_dir: std::path::PathBuf,
}

pub struct RenderContext<'a> {
    pub style: &'a StyleConfig,
    pub palette: &'a Palette,
    pub opts: &'a RenderOptions,
}

/// Render the root node into a tree string, populate `stats` and `code_files`.
pub fn render_tree(
    root: &Node,
    ctx: &RenderContext,
    stats: &mut CodeStats,
    code_files: &mut Vec<(std::path::PathBuf, String)>,
) -> String {
    let mut out = String::new();

    // Root line
    let root_name = root.name();
    let colored_name = paint_name(root_name, root.kind, ctx.palette);
    let icon = icon_str(root_name, root.kind, ctx.style);
    let icon_prefix = if ctx.style.use_icons && !icon.is_empty() {
        format!("{} ", icon)
    } else {
        String::new()
    };
    out.push_str(&format!("{}{}/\n", icon_prefix, colored_name));

    for (i, child) in root.children.iter().enumerate() {
        let is_last = i == root.children.len() - 1;
        render_node(child, "", is_last, ctx, stats, code_files, &mut out);
    }

    out
}

fn render_node(
    node: &Node,
    prefix: &str,
    is_last: bool,
    ctx: &RenderContext,
    stats: &mut CodeStats,
    code_files: &mut Vec<(std::path::PathBuf, String)>,
    out: &mut String,
) {
    let connector_raw = if is_last {
        ctx.style.branch_last()
    } else {
        ctx.style.branch_mid()
    };

    let display_name: String = if ctx.opts.full_path {
        node.path
            .strip_prefix(&ctx.opts.root_dir)
            .unwrap_or(&node.path)
            .to_string_lossy()
            .to_string()
    } else {
        node.name().to_string()
    };

    let icon = icon_str(&display_name, node.kind, ctx.style);
    let icon_prefix = if ctx.style.use_icons && !icon.is_empty() {
        format!("{} ", icon)
    } else {
        String::new()
    };
    let name_colored = paint_name(&display_name, node.kind, ctx.palette);
    let connector = paint_connector(connector_raw, ctx.palette);
    let prefix_colored = paint_connector(prefix, ctx.palette);

    let dir_slash = if node.kind == NodeKind::Dir { "/" } else { "" };

    // ── Read file content once if any flag needs it ────────────────────────
    let needs_content = node.kind == NodeKind::File
        && (ctx.opts.show_lines || ctx.opts.show_code || ctx.opts.analyze);

    let file_content: Option<String> = if needs_content {
        std::fs::read_to_string(&node.path).ok()
    } else {
        None
    };

    // Populate stats and code_files
    if let Some(ref content) = file_content {
        if ctx.opts.analyze || ctx.opts.show_lines {
            analyze::analyze_file(&node.path, content, stats);
        }
        if ctx.opts.show_code {
            code_files.push((node.path.clone(), content.clone()));
        }
    }

    // ── Build metadata string ──────────────────────────────────────────────
    let mut meta_parts: Vec<String> = Vec::new();

    if ctx.opts.show_sizes && node.kind != NodeKind::Dir {
        meta_parts.push(human_size(node.size));
    }

    if ctx.opts.show_lines && node.kind == NodeKind::File {
        let lc = file_content
            .as_deref()
            .map(|c| c.lines().count())
            .unwrap_or(0);
        meta_parts.push(format!("{} lines", lc));
    }

    let meta_str = if meta_parts.is_empty() {
        String::new()
    } else {
        format!(
            " {}",
            paint_meta(&format!("({})", meta_parts.join(", ")), ctx.palette)
        )
    };

    // ── Git glyph ─────────────────────────────────────────────────────────
    let git_glyph = if ctx.opts.show_git {
        node.git_xy
            .map(|xy| format!(" [{}]", git_status_glyph(xy, ctx.palette)))
            .unwrap_or_default()
    } else {
        String::new()
    };

    out.push_str(&format!(
        "{}{}{}{}{}{}{}\n",
        prefix_colored,
        connector,
        icon_prefix,
        name_colored,
        dir_slash,
        meta_str,
        git_glyph,
    ));

    // ── Recurse into directories ───────────────────────────────────────────
    if node.kind == NodeKind::Dir {
        let segment = if is_last {
            ctx.style.branch_blank()
        } else {
            ctx.style.branch_pipe()
        };
        let new_prefix = format!("{}{}", prefix, segment);

        for (i, child) in node.children.iter().enumerate() {
            let child_last = i == node.children.len() - 1;
            render_node(child, &new_prefix, child_last, ctx, stats, code_files, out);
        }
    }
}
