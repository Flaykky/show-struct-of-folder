use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ModeConfig {
    vertical: String,
    tee: String,
    elbow: String,
    indent: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    modes: std::collections::HashMap<String, ModeConfig>,
    default_mode: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        let mut modes = std::collections::HashMap::new();
        // old ssp mode #1
        modes.insert(
            "old".into(),
            ModeConfig {
                vertical: "│  ".into(),
                tee: "├──".into(),
                elbow: "└──".into(),
                indent: "    ".into(),
            },
        );
        // new ssp mode #2
        modes.insert(
            "new".into(),
            ModeConfig {
                vertical: "│  ".into(),
                tee: "├──".into(),
                elbow: "└──".into(),
                indent: "    ".into(),
            },
        );
        Self {
            modes,
            default_mode: Some("old".into()),
        }
    }
}

fn load_config(config_path: &Path) -> Config {
    if config_path.exists() {
        let raw = fs::read_to_string(config_path)
            .expect("Failed to read config file");
        toml::from_str(&raw).expect("Failed to parse config file")
    } else {
        Config::default()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut target_dir: PathBuf;
    let mut mode_name = None;

    // parse arguments
    for arg in &args[1..] {
        if arg.starts_with("--mode=") {
            mode_name = Some(arg.trim_start_matches("--mode=").to_string());
        } else {
            target_dir = PathBuf::from(arg);
        }
    }
    
    // find config in current dir or home
    let config_path = PathBuf::from("ssp.toml");
    let config = load_config(&config_path);
    
    let chosen_mode = mode_name
        .or_else(|| config.default_mode.clone())
        .expect("No mode specified and no default in config");
    
    let mode = config.modes.get(&chosen_mode)
        .unwrap_or_else(|| panic!("Mode '{}' not found in config", chosen_mode));

    // determine target directory
    if args.len() < 2 {
        target_dir = env::current_dir().unwrap();
    } else if target_dir.as_os_str().is_empty() {
        target_dir = env::current_dir().unwrap();
    }

    if !target_dir.exists() {
        eprintln!("Error: '{}' does not exist", target_dir.display());
        return;
    }
    if !target_dir.is_dir() {
        eprintln!("Error: '{}' is not a directory", target_dir.display());
        return;
    }

    let root_name = target_dir.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(".");
    println!("{}/", root_name);

    let mut entries: Vec<_> = fs::read_dir(&target_dir)
        .expect("Failed to read target directory")
        .map(|res| res.expect("Failed to get directory entry"))
        .collect();

    filter_and_sort_entries(&mut entries);

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let path = entry.path();
        if path.is_dir() {
            print_dir(&path, "", is_last, true, mode);
        } else {
            print_file(&path, "", is_last, true, mode);
        }
    }
}

fn filter_and_sort_entries(entries: &mut Vec<fs::DirEntry>) {
    entries.retain(|entry| {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().unwrap().to_str().unwrap();
            !name.starts_with('.') && name != "node_modules" && name != "__pycache__" && name != ".git"
        } else {
            true
        }
    });

    entries.sort_by(|a, b| {
        let a_is_dir = a.path().is_dir();
        let b_is_dir = b.path().is_dir();
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });
}

fn print_dir(
    path: &Path,
    prefix: &str,
    is_last: bool,
    is_root: bool,
    cfg: &ModeConfig,
) {
    let name = path.file_name().unwrap().to_str().unwrap();
    let connector = if is_root {
        &cfg.tee
    } else if is_last {
        &cfg.elbow
    } else {
        &cfg.tee
    };
    println!("{}{} {}", prefix, connector, name);

    let new_prefix = if is_root || is_last {
        format!("{}{}", prefix, cfg.indent)
    } else {
        format!("{}{}", prefix, cfg.vertical)
    };

    let mut entries: Vec<_> = fs::read_dir(path)
        .expect("Failed to read directory")
        .map(|res| res.expect("Failed to get directory entry"))
        .collect();

    filter_and_sort_entries(&mut entries);

    for (i, entry) in entries.iter().enumerate() {
        let is_last_entry = i == entries.len() - 1;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            print_dir(&entry_path, &new_prefix, is_last_entry, false, cfg);
        } else {
            print_file(&entry_path, &new_prefix, is_last_entry, false, cfg);
        }
    }
}

fn print_file(
    path: &Path,
    prefix: &str,
    is_last: bool,
    _is_root: bool,
    cfg: &ModeConfig,
) {
    let name = path.file_name().unwrap().to_str().unwrap();
    let connector = if is_last { &cfg.elbow } else { &cfg.tee };
    println!("{}{} {}", prefix, connector, name);
}
