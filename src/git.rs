//! Git status integration: per-path status via `git status --porcelain`.
//! Gracefully degrades if git is not installed or the dir is not a repo.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Key: canonical path (as reported by git, relative to repo root joined with root).
/// Value: [index_byte, worktree_byte].
pub type GitStatusMap = HashMap<PathBuf, [u8; 2]>;

/// Run `git status --porcelain=v1 -z` in `dir` and parse the output.
/// Returns `None` if git is unavailable or `dir` is not a repo.
pub fn collect_status(dir: &Path) -> Option<GitStatusMap> {
    let output = Command::new("git")
        .args(["status", "--porcelain=v1", "-z", "--ignored=matching"])
        .current_dir(dir)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    // Find the repo root so we can build absolute paths.
    let root_output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(dir)
        .output()
        .ok()?;
    let root = PathBuf::from(
        String::from_utf8_lossy(&root_output.stdout)
            .trim()
            .to_string(),
    );

    let mut map = GitStatusMap::new();
    // Porcelain v1 with -z: entries separated by NUL. Each entry is "XY path".
    let stdout = output.stdout;
    let mut iter = stdout.split(|&b| b == 0).peekable();

    while let Some(entry) = iter.next() {
        if entry.len() < 4 {
            continue;
        }
        let xy = [entry[0], entry[1]];
        // entry[2] is a space, entry[3..] is the path (or new name for renames)
        let rel_path = match std::str::from_utf8(&entry[3..]) {
            Ok(s) => s,
            Err(_) => continue,
        };
        // For renames: "XY new_name\0orig_name" — we use new_name (already in rel_path).
        let abs_path = root.join(rel_path);
        map.insert(abs_path, xy);
    }

    Some(map)
}
