//! In-memory tree model and tree-shape rendering.

use std::path::PathBuf;

/// Broad kind of a filesystem entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Dir,
    File,
    Symlink,
}

/// A single node in the directory tree.
#[derive(Debug)]
pub struct Node {
    /// Absolute path.
    pub path: PathBuf,
    pub kind: NodeKind,
    /// Children (directories appear before files after sorting).
    pub children: Vec<Node>,
    /// Cached metadata.
    pub size: u64,
    /// Modification time as Unix seconds (0 if unavailable).
    pub mtime: i64,
    /// Whether the file is executable (Unix only; always false on Windows).
    #[allow(dead_code)]
    pub is_exec: bool,
    /// Git status bytes [index, worktree] — 0x20 (' ') means clean.
    pub git_xy: Option<[u8; 2]>,
}

impl Node {
    pub fn name(&self) -> &str {
        self.path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("<invalid>")
    }

    pub fn extension(&self) -> &str {
        self.path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
    }

    /// Total file count under this node (recursively).
    pub fn file_count(&self) -> usize {
        if self.kind != NodeKind::Dir {
            return 1;
        }
        self.children.iter().map(|c| c.file_count()).sum()
    }

    /// Total dir count under this node (recursively, not counting self).
    pub fn dir_count(&self) -> usize {
        if self.kind != NodeKind::Dir {
            return 0;
        }
        self.children
            .iter()
            .map(|c| if c.kind == NodeKind::Dir { 1 + c.dir_count() } else { 0 })
            .sum()
    }

    /// Recursive byte total.
    pub fn total_size(&self) -> u64 {
        if self.kind != NodeKind::Dir {
            return self.size;
        }
        self.children.iter().map(|c| c.total_size()).sum()
    }
}

/// Human-readable byte size.
pub fn human_size(bytes: u64) -> String {
    const K: u64 = 1024;
    if bytes < K {
        format!("{}B", bytes)
    } else if bytes < K * K {
        format!("{:.1}K", bytes as f64 / K as f64)
    } else if bytes < K * K * K {
        format!("{:.1}M", bytes as f64 / (K * K) as f64)
    } else {
        format!("{:.1}G", bytes as f64 / (K * K * K) as f64)
    }
}
