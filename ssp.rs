use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_dir: PathBuf;

    if args.len() < 2 {
        // Не передан аргумент — используем текущую директорию
        target_dir = env::current_dir().unwrap();
    } else {
        let path_arg = &args[1];
        let path = Path::new(path_arg);

        if !path.exists() {
            eprintln!("Ошибка: Путь '{}' не существует", path_arg);
            return;
        }

        if !path.is_dir() {
            eprintln!("Ошибка: '{}' не является директорией", path_arg);
            return;
        }

        target_dir = path.to_path_buf();
    }

    let root_name = target_dir.file_name().and_then(|s| s.to_str()).unwrap_or(".");
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
            print_dir_structure(&path, "", is_last, true);
        } else {
            print_file_structure(&path, "", is_last, true);
        }
    }
}

fn filter_and_sort_entries(entries: &mut Vec<std::fs::DirEntry>) {
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

fn print_dir_structure(path: &Path, prefix: &str, is_last: bool, is_root: bool) {
    let name = path.file_name().unwrap().to_str().unwrap();
    let connector = if is_root { "│──" } else if is_last { "└──" } else { "├──" };
    let new_prefix_segment = if is_root || is_last { "    " } else { "│   " };

    println!("{}{} {}", prefix, connector, name);

    let new_prefix = format!("{}{}", prefix, new_prefix_segment);

    let mut entries: Vec<_> = fs::read_dir(path)
        .expect("Failed to read directory")
        .map(|res| res.expect("Failed to get directory entry"))
        .collect();

    filter_and_sort_entries(&mut entries);

    for (i, entry) in entries.iter().enumerate() {
        let is_last_entry = i == entries.len() - 1;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            print_dir_structure(&entry_path, &new_prefix, is_last_entry, false);
        } else {
            print_file_structure(&entry_path, &new_prefix, is_last_entry, false);
        }
    }
}

fn print_file_structure(path: &Path, prefix: &str, is_last: bool, is_root: bool) {
    let name = path.file_name().unwrap().to_str().unwrap();
    let connector = if is_root || is_last { "└──" } else { "├──" };
    println!("{}{} {}", prefix, connector, name);
}