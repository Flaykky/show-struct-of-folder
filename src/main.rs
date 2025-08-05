use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Config {
    target_dir: PathBuf,
    ignore_folders: HashSet<String>,
    only_folders: bool,
    show_lines: bool,
    only_extension: Option<String>,
    max_depth: Option<usize>,
}

impl Config {
    fn new() -> Self {
        Self {
            target_dir: env::current_dir().unwrap(),
            ignore_folders: HashSet::new(),
            only_folders: false,
            show_lines: false,
            only_extension: None,
            max_depth: None,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config = Config::new();
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--ignore" | "-i" => {
                if i + 1 < args.len() {
                    config.ignore_folders.insert(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --ignore flag requires an argument");
                    return;
                }
            }
            "--only-folders" | "-of" => {
                config.only_folders = true;
                i += 1;
            }
            "--lines" | "-l" => {
                config.show_lines = true;
                i += 1;
            }
            "--extension" | "-e" => {
                if i + 1 < args.len() {
                    config.only_extension = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --extension flag requires an argument");
                    return;
                }
            }
            "--depth" | "-d" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<usize>() {
                        Ok(depth) => {
                            config.max_depth = Some(depth);
                            i += 2;
                        }
                        Err(_) => {
                            eprintln!("Error: --depth flag requires a numeric value");
                            return;
                        }
                    }
                } else {
                    eprintln!("Error: --depth flag requires an argument");
                    return;
                }
            }
            "--help" | "-h" => {
                print_help();
                return;
            }
            arg if !arg.starts_with('-') => {
                let path = Path::new(arg);
                if !path.exists() {
                    eprintln!("Error: Path '{}' does not exist", arg);
                    return;
                }
                if !path.is_dir() {
                    eprintln!("Error: '{}' is not a directory", arg);
                    return;
                }
                config.target_dir = path.to_path_buf();
                i += 1;
            }
            _ => {
                eprintln!("Unknown flag: {}", args[i]);
                print_help();
                return;
            }
        }
    }

    // Add default ignored folders
    let default_ignores = vec![".git", "node_modules", "__pycache__"];
    for ignore in default_ignores {
        if !config.ignore_folders.contains(ignore) {
            config.ignore_folders.insert(ignore.to_string());
        }
    }

    display_structure(&config);
}

fn print_help() {
    println!("Usage: ssp [options] [directory_path]");
    println!();
    println!("Options:");
    println!("  -i, --ignore FOLDER     Ignore the specified folder");
    println!("  -of, --only-folders     Show only folders");
    println!("  -l, --lines             Show the number of lines in files");
    println!("  -e, --extension EXT     Show only files with the specified extension");
    println!("  -d, --depth DEPTH       Limit the display depth");
    println!("  -h, --help              Show this help message");
    println!();
    println!("Examples:");
    println!("  ssp                     Display the structure of the current directory");
    println!("  ssp /path/to/dir        Display the structure of the specified directory");
    println!("  ssp -i target -of       Only folders, ignore 'target'");
    println!("  ssp -l -e rs            .rs files with line counts");
    println!("  ssp -d 2                Show structure up to 2 levels deep");
}

fn display_structure(config: &Config) {
    let root_name = config.target_dir.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(".");

    println!("{}/", root_name);

    let mut entries: Vec<_> = fs::read_dir(&config.target_dir)
        .expect("Failed to read target directory")
        .map(|res| res.expect("Failed to get directory entry"))
        .collect();

    filter_and_sort_entries(&mut entries, config);

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let path = entry.path();
        if path.is_dir() {
            print_dir_structure(&path, "", is_last, 0, config);
        } else {
            print_file_structure(&path, "", is_last, config);
        }
    }
}

// Filter and sort directory entries
fn filter_and_sort_entries(entries: &mut Vec<std::fs::DirEntry>, config: &Config) {
    entries.retain(|entry| {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().unwrap().to_str().unwrap();
            // Check ignored folders
            if config.ignore_folders.contains(name) {
                return false;
            }
            true
        } else {
            // Show only folders if specified
            if config.only_folders {
                return false;
            }
            
            // Show only files with the specified extension
            if let Some(ref ext) = config.only_extension {
                if let Some(file_ext) = path.extension() {
                    if file_ext.to_str() != Some(ext) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
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

fn print_dir_structure(
    path: &Path, 
    prefix: &str, 
    is_last: bool, 
    current_depth: usize,
    config: &Config
) {
    let name = path.file_name().unwrap().to_str().unwrap();
    let connector = if is_last { "└──" } else { "├──" };
    let new_prefix_segment = if is_last { "    " } else { "│   " };

    println!("{}{} {}", prefix, connector, name);

    // Check depth limit
    if let Some(max_depth) = config.max_depth {
        if current_depth >= max_depth {
            return;
        }
    }

    let new_prefix = format!("{}{}", prefix, new_prefix_segment);

    let mut entries: Vec<_> = fs::read_dir(path)
        .expect("Failed to read directory")
        .map(|res| res.expect("Failed to get directory entry"))
        .collect();

    filter_and_sort_entries(&mut entries, config);

    for (i, entry) in entries.iter().enumerate() {
        let is_last_entry = i == entries.len() - 1;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            print_dir_structure(&entry_path, &new_prefix, is_last_entry, current_depth + 1, config);
        } else {
            print_file_structure(&entry_path, &new_prefix, is_last_entry, config);
        }
    }
}

fn print_file_structure(path: &Path, prefix: &str, is_last: bool, config: &Config) {
    let name = path.file_name().unwrap().to_str().unwrap();
    let connector = if is_last { "└──" } else { "├──" };
    
    if config.show_lines {
        let line_count = count_lines(path);
        println!("{}{} {} ({})", prefix, connector, name, line_count);
    } else {
        println!("{}{} {}", prefix, connector, name);
    }
}

fn count_lines(path: &Path) -> usize {
    if let Ok(content) = fs::read_to_string(path) {
        content.lines().count()
    } else {
        0
    }
}