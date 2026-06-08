//! Directory traversal using the `ignore` crate's WalkBuilder.
//! Builds an in-memory Node tree.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use ignore::WalkBuilder;

use crate::cli::SortKey;
use crate::git::GitStatusMap;
use crate::tree::{Node, NodeKind};

/// Parameters that govern what the walker includes/excludes and how it sorts.
pub struct WalkOptions<'a> {
    pub root: &'a Path,
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
    let entries = read_child_entries(dir, opts);

    let mut children: Vec<Node> = entries
        .into_iter()
        .filter(|entry| should_include_entry(entry, opts))
        .map(|entry| build_node(&entry.path, opts, depth))
        .collect();

    sort_children(&mut children, opts);
    children
}

#[derive(Debug)]
struct ChildEntry {
    path: PathBuf,
    name: String,
    is_dir: bool,
}

fn read_child_entries(dir: &Path, opts: &WalkOptions) -> Vec<ChildEntry> {
    if opts.respect_gitignore {
        let mut builder = WalkBuilder::new(dir);
        builder
            .max_depth(Some(1))
            .hidden(false)
            .ignore(true)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true);

        return builder
            .build()
            .filter_map(|res| res.ok())
            .filter(|entry| entry.depth() == 1)
            .filter_map(|entry| {
                let path = entry.into_path();
                entry_from_path(path)
            })
            .collect();
    }

    match std::fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(|res| res.ok())
            .filter_map(|entry| entry_from_path(entry.path()))
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn entry_from_path(path: PathBuf) -> Option<ChildEntry> {
    let name = path.file_name()?.to_string_lossy().to_string();
    let is_dir = std::fs::symlink_metadata(&path)
        .map(|m| m.is_dir())
        .unwrap_or(false);
    Some(ChildEntry { path, name, is_dir })
}

fn should_include_entry(entry: &ChildEntry, opts: &WalkOptions) -> bool {
    if !opts.show_hidden && entry.name.starts_with('.') {
        return false;
    }

    if opts.ignore_names.contains(entry.name.as_str()) {
        return false;
    }

    if opts.dirs_only && !entry.is_dir {
        return false;
    }
    if opts.files_only && entry.is_dir {
        return false;
    }

    if !entry.is_dir {
        if let Some(ext) = opts.extension_filter {
            let file_ext = entry.path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if file_ext != ext.trim_start_matches('.') {
                return false;
            }
        }
    }

    if !opts.include_globs.is_empty()
        && !entry.is_dir
        && !opts
            .include_globs
            .iter()
            .any(|pat| glob_match_entry(pat, &entry.name, &entry.path, opts.root))
    {
        return false;
    }

    if opts
        .exclude_globs
        .iter()
        .any(|pat| glob_match_entry(pat, &entry.name, &entry.path, opts.root))
    {
        return false;
    }

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

fn glob_match_entry(pattern: &str, name: &str, path: &Path, root: &Path) -> bool {
    if glob_match(pattern, name) {
        return true;
    }

    let rel = path
        .strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");
    glob_match(pattern, &rel)
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
    use super::{build_tree, glob_match, WalkOptions};
    use crate::cli::SortKey;
    use crate::tree::NodeKind;
    use std::collections::HashSet;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_glob() {
        assert!(glob_match("*.rs", "main.rs"));
        assert!(glob_match("*.rs", "foo.rs"));
        assert!(!glob_match("*.rs", "foo.py"));
        assert!(glob_match("foo*", "foobar"));
        assert!(glob_match("f?o", "foo"));
        assert!(!glob_match("f?o", "fo"));
    }

    #[test]
    fn respects_gitignore_without_hiding_dotfiles_when_all_is_set() {
        let root = temp_root("ssp-gitignore");
        fs::write(root.join(".gitignore"), "ignored.txt\n").unwrap();
        fs::write(root.join(".env"), "SECRET=1\n").unwrap();
        fs::write(root.join("ignored.txt"), "ignored\n").unwrap();
        fs::write(root.join("kept.txt"), "kept\n").unwrap();

        let ignore_names = HashSet::new();
        let opts = test_opts(&root, true, true, &ignore_names, &[], &[]);
        let tree = build_tree(&root, &opts);
        let names = child_names(&tree);

        assert!(names.contains(&".env".to_string()));
        assert!(names.contains(&"kept.txt".to_string()));
        assert!(!names.contains(&"ignored.txt".to_string()));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn applies_parent_gitignore_rules_to_nested_entries() {
        let root = temp_root("ssp-parent-gitignore");
        fs::create_dir(root.join("src")).unwrap();
        fs::write(root.join(".gitignore"), "src/generated.rs\n").unwrap();
        fs::write(root.join("src/generated.rs"), "ignored\n").unwrap();
        fs::write(root.join("src/main.rs"), "fn main() {}\n").unwrap();

        let ignore_names = HashSet::new();
        let opts = test_opts(&root, true, true, &ignore_names, &[], &[]);
        let tree = build_tree(&root, &opts);
        let src = tree.children.iter().find(|node| node.name() == "src").unwrap();
        let names = child_names(src);

        assert_eq!(names, vec!["main.rs".to_string()]);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn extension_filter_accepts_leading_dot() {
        let root = temp_root("ssp-extension");
        fs::write(root.join("main.rs"), "fn main() {}\n").unwrap();
        fs::write(root.join("main.ts"), "console.log(1)\n").unwrap();

        let ignore_names = HashSet::new();
        let mut opts = test_opts(&root, false, false, &ignore_names, &[], &[]);
        opts.extension_filter = Some(".rs");

        let tree = build_tree(&root, &opts);
        let names = child_names(&tree);

        assert_eq!(names, vec!["main.rs".to_string()]);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn exclude_globs_match_relative_paths() {
        let root = temp_root("ssp-relative-glob");
        fs::create_dir(root.join("src")).unwrap();
        fs::write(root.join("src/main.rs"), "fn main() {}\n").unwrap();
        fs::write(root.join("README.md"), "# SSP\n").unwrap();

        let exclude = vec!["src/*.rs".to_string()];
        let ignore_names = HashSet::new();
        let opts = test_opts(&root, false, false, &ignore_names, &[], &exclude);

        let tree = build_tree(&root, &opts);
        let src = tree.children.iter().find(|node| node.name() == "src").unwrap();

        assert_eq!(src.kind, NodeKind::Dir);
        assert!(src.children.is_empty());

        fs::remove_dir_all(root).unwrap();
    }

    fn test_opts<'a>(
        root: &'a Path,
        show_hidden: bool,
        respect_gitignore: bool,
        ignore_names: &'a HashSet<String>,
        include_globs: &'a [String],
        exclude_globs: &'a [String],
    ) -> WalkOptions<'a> {
        WalkOptions {
            root,
            max_depth: None,
            show_hidden,
            respect_gitignore,
            ignore_names,
            include_globs,
            exclude_globs,
            extension_filter: None,
            dirs_only: false,
            files_only: false,
            prune: false,
            sort: SortKey::Name,
            reverse: false,
            dirs_first: true,
            git_status: None,
        }
    }

    fn child_names(node: &crate::tree::Node) -> Vec<String> {
        node.children
            .iter()
            .map(|child| child.name().to_string())
            .collect()
    }

    fn temp_root(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("{name}-{nanos}"));
        fs::create_dir(&root).unwrap();
        root
    }
}
