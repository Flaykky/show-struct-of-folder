//! Directory traversal using the `ignore` crate's WalkBuilder.
//! Builds an in-memory Node tree.

use std::collections::HashSet;
use std::path::Path;

use ignore::WalkBuilder;

use crate::cli::SortKey;
use crate::git::GitStatusMap;
use crate::tree::{Node, NodeKind};

/// Parameters that govern what the walker includes/excludes and how it sorts.
pub struct WalkOptions<'a> {
    pub max_depth: Option<usize>,
    pub show_hidden: bool,
    pub respect_gitignore: bool,
    pub ignore_names: &'a HashSet<String>,
    pub include_globs: &'a [String],
    pub exclude_globs: &'a [String],
    pub extension_filter: Option<&'a str>,
    pub dirs_only: bool,
    pub files_only: bool,
    pub prune: bool,
    pub sort: SortKey,
    pub reverse: bool,
    pub dirs_first: bool,
    pub git_status: Option<&'a GitStatusMap>,
}

/// Build a `Node` tree rooted at `root`.
pub fn build_tree(root: &Path, opts: &WalkOptions) -> Node {
    let mut root_node = build_node(root, opts, 0);
    if opts.prune {
        prune_empty_dirs(&mut root_node);
    }
    root_node
}

fn build_node(path: &Path, opts: &WalkOptions, depth: usize) -> Node {
    let meta = std::fs::symlink_metadata(path).ok();

    let kind = if let Some(ref m) = meta {
        if m.file_type().is_symlink() {
            NodeKind::Symlink
        } else if m.is_dir() {
            NodeKind::Dir
        } else {
            NodeKind::File
        }
    } else {
        NodeKind::File
    };

    let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
    let mtime = meta.as_ref()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    #[cfg(unix)]
    let is_exec = meta.as_ref()
        .map(|m| {
            use std::os::unix::fs::PermissionsExt;
            m.permissions().mode() & 0o111 != 0
        })
        .unwrap_or(false);
    #[cfg(not(unix))]
    let is_exec = false;

    let git_xy = opts.git_status.and_then(|map| map.get(path).copied());

    let mut node = Node {
        path: path.to_path_buf(),
        kind,
        children: Vec::new(),
        size,
        mtime,
        is_exec,
        git_xy,
    };

    if kind == NodeKind::Dir {
        if let Some(max) = opts.max_depth {
            if depth >= max {
                return node;
            }
        }
        node.children = read_children(path, opts, depth + 1);
    }

    node
}

fn read_children(dir: &Path, opts: &WalkOptions, depth: usize) -> Vec<Node> {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };

    let mut children: Vec<Node> = entries
        .filter_map(|res| res.ok())
        .filter(|entry| {
            let name_os = entry.file_name();
            let name = name_os.to_string_lossy();
            let path = entry.path();

            // Hidden files
            if !opts.show_hidden && name.starts_with('.') {
                return false;
            }

            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);

            // Ignored names
            if opts.ignore_names.contains(name.as_ref()) {
                return false;
            }

            // Gitignore — delegate to ignore crate for files inside a walk
            // (We handle this via WalkBuilder for the root; for children we do
            //  a simple inline check using the ignore crate's matcher.)
            // Actually we build per-entry below using ignore crate.

            // dirs_only / files_only
            if opts.dirs_only && !is_dir {
                return false;
            }
            if opts.files_only && is_dir {
                return false;
            }

            // Extension filter (files only)
            if !is_dir {
                if let Some(ext) = opts.extension_filter {
                    let file_ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                    if file_ext != ext {
                        return false;
                    }
                }
            }

            // Include globs (files only, skip dirs so we can recurse)
            if !opts.include_globs.is_empty() && !is_dir {
                let matched = opts.include_globs.iter().any(|pat| {
                    glob_match(pat, name.as_ref())
                });
                if !matched {
                    return false;
                }
            }

            // Exclude globs
            if !opts.exclude_globs.is_empty() {
                let excluded = opts.exclude_globs.iter().any(|pat| {
                    glob_match(pat, name.as_ref())
                });
                if excluded {
                    return false;
                }
            }

            true
        })
        .filter(|entry| {
            // Gitignore via ignore crate's standalone matcher
            if opts.respect_gitignore {
                !is_gitignored(&entry.path())
            } else {
                true
            }
        })
        .map(|entry| build_node(&entry.path(), opts, depth))
        .collect();

    sort_children(&mut children, opts);
    children
}

/// Minimal gitignore check using the `ignore` crate's WalkBuilder on a single path.
fn is_gitignored(path: &Path) -> bool {
    // Build a one-shot walker; if it yields the path, it's not ignored.
    // This is slightly heavy but correct and cross-platform.
    // For large trees the git status map approach would be faster; good enough for now.
    let parent = match path.parent() {
        Some(p) => p,
        None => return false,
    };
    let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();

    // Use WalkBuilder to test this single entry.
    let mut builder = WalkBuilder::new(parent);
    builder
        .max_depth(Some(1))
        .hidden(false)
        .ignore(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true);

    for entry in builder.build().flatten() {
        if entry.path() == path || entry.file_name() == std::ffi::OsStr::new(&name) {
            // The walker yielded it → not ignored.
            return false;
        }
    }
    // Wasn't yielded → treated as ignored.
    true
}

fn sort_children(children: &mut Vec<Node>, opts: &WalkOptions) {
    let dirs_first = opts.dirs_first && !opts.dirs_only && !opts.files_only;

    children.sort_by(|a, b| {
        // Dirs first?
        if dirs_first {
            let a_dir = a.kind == NodeKind::Dir;
            let b_dir = b.kind == NodeKind::Dir;
            if a_dir && !b_dir {
                return std::cmp::Ordering::Less;
            }
            if !a_dir && b_dir {
                return std::cmp::Ordering::Greater;
            }
        }

        let ord = match opts.sort {
            SortKey::Name | SortKey::None => a.name().cmp(b.name()),
            SortKey::Size => a.size.cmp(&b.size),
            SortKey::Time => a.mtime.cmp(&b.mtime),
            SortKey::Ext => {
                let ea = a.extension();
                let eb = b.extension();
                ea.cmp(eb).then(a.name().cmp(b.name()))
            }
        };

        if opts.reverse { ord.reverse() } else { ord }
    });
}

/// Remove directories that have no visible children (after filtering).
fn prune_empty_dirs(node: &mut Node) {
    if node.kind != NodeKind::Dir {
        return;
    }
    node.children.retain_mut(|child| {
        prune_empty_dirs(child);
        !(child.kind == NodeKind::Dir && child.children.is_empty())
    });
}

/// Extremely simple glob matcher supporting `*` (any chars except `/`) and `?`.
fn glob_match(pattern: &str, name: &str) -> bool {
    let pat: Vec<char> = pattern.chars().collect();
    let txt: Vec<char> = name.chars().collect();
    glob_match_inner(&pat, &txt)
}

fn glob_match_inner(pat: &[char], txt: &[char]) -> bool {
    match (pat.first(), txt.first()) {
        (None, None) => true,
        (None, _) | (Some('?'), None) => false,
        (Some('*'), _) => {
            // * matches zero or more chars (excluding '/')
            if glob_match_inner(&pat[1..], txt) {
                return true;
            }
            if txt.first() == Some(&'/') {
                return false;
            }
            !txt.is_empty() && glob_match_inner(pat, &txt[1..])
        }
        (Some('?'), _) => glob_match_inner(&pat[1..], &txt[1..]),
        (Some(p), Some(t)) => p == t && glob_match_inner(&pat[1..], &txt[1..]),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::glob_match;

    #[test]
    fn test_glob() {
        assert!(glob_match("*.rs", "main.rs"));
        assert!(glob_match("*.rs", "foo.rs"));
        assert!(!glob_match("*.rs", "foo.py"));
        assert!(glob_match("foo*", "foobar"));
        assert!(glob_match("f?o", "foo"));
        assert!(!glob_match("f?o", "fo"));
    }
}
