//! Code analysis and show-code extraction.

use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Default)]
pub struct CodeStats {
    pub total_lines: usize,
    pub total_files: usize,
    pub files_by_extension: HashMap<String, usize>,
    pub lines_by_extension: HashMap<String, usize>,
    pub blank_lines: usize,
    pub comment_lines: usize,
    pub code_lines: usize,
    // Approximate declaration counts
    pub int_count: usize,
    pub float_count: usize,
    pub string_count: usize,
    pub bool_count: usize,
    pub function_count: usize,
    pub class_count: usize,
}

/// Language comment styles.
enum CommentStyle {
    /// Double-slash and block-comment lines
    CStyle,
    /// Hash prefix
    Hash,
    /// Double-dash
    DoubleDash,
    /// Percent
    Percent,
    /// Semicolon
    Semicolon,
    Unknown,
}

fn comment_style(ext: &str) -> CommentStyle {
    match ext {
        "rs" | "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "hxx" | "java" | "kt" | "kts"
        | "go" | "cs" | "js" | "ts" | "jsx" | "tsx" | "swift" | "dart" | "zig" | "v"
        | "scala" | "groovy" | "d" | "vala" => CommentStyle::CStyle,
        "py" | "rb" | "sh" | "bash" | "zsh" | "fish" | "ksh" | "pl" | "pm" | "yaml" | "yml"
        | "toml" | "r" | "rmd" | "cr" | "nim" | "elixir" | "ex" | "exs" | "makefile"
        | "conf" | "cfg" | "ini" => CommentStyle::Hash,
        "lua" | "hs" | "lhs" | "sql" | "ada" => CommentStyle::DoubleDash,
        "m" | "matlab" | "tex" | "latex" => CommentStyle::Percent,
        "lisp" | "clj" | "cljs" | "el" | "scm" | "ss" => CommentStyle::Semicolon,
        _ => CommentStyle::Unknown,
    }
}

fn is_comment_line(trimmed: &str, ext: &str) -> bool {
    match comment_style(ext) {
        CommentStyle::CStyle => {
            trimmed.starts_with("//")
                || trimmed.starts_with("/*")
                || trimmed.starts_with('*')
        }
        CommentStyle::Hash => trimmed.starts_with('#'),
        CommentStyle::DoubleDash => trimmed.starts_with("--"),
        CommentStyle::Percent => trimmed.starts_with('%'),
        CommentStyle::Semicolon => trimmed.starts_with(';'),
        CommentStyle::Unknown => false,
    }
}

pub fn analyze_file(path: &Path, content: &str, stats: &mut CodeStats) {
    stats.total_files += 1;

    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    *stats.files_by_extension.entry(ext.clone()).or_insert(0) += 1;

    let mut line_count = 0usize;

    for line in content.lines() {
        line_count += 1;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            stats.blank_lines += 1;
            continue;
        }

        if is_comment_line(trimmed, &ext) {
            stats.comment_lines += 1;
            continue;
        }

        // Code line
        stats.code_lines += 1;

        // --- approximate type declarations ---
        // Integer types (language-sensitive would be ideal; this is "approximate")
        if trimmed.contains(": i32")
            || trimmed.contains(": i64")
            || trimmed.contains(": i128")
            || trimmed.contains(": isize")
            || trimmed.contains(": u32")
            || trimmed.contains(": u64")
            || trimmed.contains(": usize")
            || trimmed.contains("int ")
            || trimmed.contains("int\t")
            || trimmed.starts_with("int ")
        {
            stats.int_count += 1;
        }
        if trimmed.contains(": f32")
            || trimmed.contains(": f64")
            || trimmed.contains("float ")
            || trimmed.contains("double ")
        {
            stats.float_count += 1;
        }
        if trimmed.contains(": String")
            || trimmed.contains(": &str")
            || trimmed.contains(": str")
            || trimmed.contains("string ")
            || trimmed.contains("String ")
            || (trimmed.contains("str ") && !trimmed.contains("struct"))
        {
            stats.string_count += 1;
        }
        if trimmed.contains(": bool")
            || trimmed.contains("bool ")
            || trimmed.contains("boolean ")
            || trimmed.contains("Bool ")
        {
            stats.bool_count += 1;
        }

        // --- functions / classes ---
        let is_fn = trimmed.starts_with("fn ")
            || trimmed.starts_with("pub fn ")
            || trimmed.starts_with("async fn ")
            || trimmed.starts_with("pub async fn ")
            || trimmed.starts_with("def ")
            || trimmed.starts_with("function ")
            || trimmed.starts_with("func ")
            || (trimmed.starts_with("fun ")   // Kotlin
                || trimmed.starts_with("pub fun "));
        if is_fn {
            stats.function_count += 1;
        }

        let is_class = trimmed.starts_with("class ")
            || trimmed.starts_with("struct ")
            || trimmed.starts_with("impl ")
            || trimmed.starts_with("pub impl ")
            || trimmed.starts_with("trait ")
            || trimmed.starts_with("pub trait ")
            || trimmed.starts_with("enum ")
            || trimmed.starts_with("pub enum ")
            || trimmed.starts_with("interface ")
            || trimmed.starts_with("abstract class ")
            || trimmed.starts_with("object ");   // Kotlin / Scala singleton
        if is_class {
            stats.class_count += 1;
        }
    }

    stats.total_lines += line_count;
    *stats.lines_by_extension.entry(ext).or_insert(0) += line_count;
}

pub fn format_analysis(stats: &CodeStats) -> String {
    let mut out = String::new();

    out.push_str(&format!("Total Files:   {}\n", stats.total_files));
    out.push_str(&format!("Total Lines:   {}\n", stats.total_lines));
    out.push_str(&format!("Blank Lines:   {}\n", stats.blank_lines));
    out.push_str(&format!("Comment Lines: {}\n", stats.comment_lines));
    out.push_str(&format!("Code Lines:    {}\n", stats.code_lines));

    if stats.total_lines > 0 {
        let pct = stats.code_lines as f64 / stats.total_lines as f64 * 100.0;
        out.push_str(&format!("Code Density:  {:.1}%\n", pct));
    }

    out.push_str("\nFiles by Extension:\n");
    let mut ext_vec: Vec<_> = stats.files_by_extension.iter().collect();
    ext_vec.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (ext, count) in &ext_vec {
        out.push_str(&format!("  .{:<10} {} files\n", ext, count));
    }

    out.push_str("\nLines by Extension:\n");
    let mut lines_vec: Vec<_> = stats.lines_by_extension.iter().collect();
    lines_vec.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (ext, count) in &lines_vec {
        out.push_str(&format!("  .{:<10} {} lines\n", ext, count));
    }

    out.push_str("\nCode Elements (approximate):\n");
    out.push_str(&format!("  Functions:        {}\n", stats.function_count));
    out.push_str(&format!("  Classes/Structs:  {}\n", stats.class_count));
    out.push_str(&format!("  Int declarations: {}\n", stats.int_count));
    out.push_str(&format!("  Float decls:      {}\n", stats.float_count));
    out.push_str(&format!("  String decls:     {}\n", stats.string_count));
    out.push_str(&format!("  Bool decls:       {}\n", stats.bool_count));

    out
}
