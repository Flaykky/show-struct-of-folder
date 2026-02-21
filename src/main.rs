use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Debug)]
struct Config {
    target_dir: PathBuf,
    ignore_folders: HashSet<String>,
    only_folders: bool,
    show_lines: bool,
    only_extension: Option<String>,
    max_depth: Option<usize>,
    output_file: Option<String>,
    show_code: bool,
    analyze_code: bool,
}

impl Config {
    fn new() -> Self {
        Self {
            target_dir: env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            ignore_folders: HashSet::new(),
            only_folders: false,
            show_lines: false,
            only_extension: None,
            max_depth: None,
            output_file: None,
            show_code: false,
            analyze_code: false,
        }
    }
}

#[derive(Debug, Default)]
struct CodeStats {
    total_lines: usize,
    total_files: usize,
    files_by_extension: HashMap<String, usize>,
    lines_by_extension: HashMap<String, usize>,
    int_count: usize,
    float_count: usize,
    string_count: usize,
    bool_count: usize,
    function_count: usize,
    class_count: usize,
    comment_lines: usize,
    blank_lines: usize,
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
                    eprintln!("Error: --ignore requires an argument");
                    process::exit(1);
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
                    eprintln!("Error: --extension requires an argument");
                    process::exit(1);
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
                            eprintln!("Error: --depth requires a numeric value");
                            process::exit(1);
                        }
                    }
                } else {
                    eprintln!("Error: --depth requires an argument");
                    process::exit(1);
                }
            }
            "--output" | "-o" => {
                if i + 1 < args.len() {
                    config.output_file = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --output requires an argument");
                    process::exit(1);
                }
            }
            "--show-code" | "-sc" => {
                config.show_code = true;
                i += 1;
            }
            "--analyze" | "-a" => {
                config.analyze_code = true;
                i += 1;
            }
            "--help" | "-h" => {
                print_help();
                return;
            }
            arg if !arg.starts_with('-') => {
                let path = Path::new(arg);
                if !path.exists() {
                    eprintln!("Error: Path '{}' does not exist", arg);
                    process::exit(1);
                }
                if !path.is_dir() {
                    eprintln!("Error: '{}' is not a directory", arg);
                    process::exit(1);
                }
                config.target_dir = path.to_path_buf();
                i += 1;
            }
            _ => {
                eprintln!("Unknown flag: {}", args[i]);
                print_help();
                process::exit(1);
            }
        }
    }

    // Add default ignored folders (insert is a no-op if already present)
    for ignore in [".git", "node_modules", "__pycache__", "target", ".idea", ".vscode"] {
        config.ignore_folders.insert(ignore.to_string());
    }

    let mut output = String::new();
    let mut code_files: Vec<(PathBuf, String)> = Vec::new();
    let mut stats = CodeStats::default();

    display_structure(&config, &mut output, &mut code_files, &mut stats);

    if config.show_code && !code_files.is_empty() {
        output.push_str("\n\n=== CODE CONTENT ===\n\n");
        for (idx, (path, content)) in code_files.iter().enumerate() {
            let relative_path = path
                .strip_prefix(&config.target_dir)
                .unwrap_or(path)
                .to_str()
                .unwrap_or("");
            output.push_str(&format!("{}. {}:\n\n", idx + 1, relative_path));
            output.push_str(content);
            output.push_str("\n\n");
            output.push_str(&"-".repeat(80));
            output.push_str("\n\n");
        }
    }

    if config.analyze_code {
        output.push_str("\n\n=== CODE ANALYSIS ===\n\n");
        output.push_str(&format_analysis(&stats));
    }

    if let Some(filename) = &config.output_file {
        match fs::File::create(filename) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(output.as_bytes()) {
                    eprintln!("Error writing to file '{}': {}", filename, e);
                    process::exit(1);
                }
                println!("Output saved to: {}", filename);
            }
            Err(e) => {
                eprintln!("Error creating file '{}': {}", filename, e);
                process::exit(1);
            }
        }
    } else {
        print!("{}", output);
    }
}

fn print_help() {
    println!("SSP - Show Structure of Project v2.0");
    println!();
    println!("Usage: ssp [options] [directory_path]");
    println!();
    println!("Options:");
    println!("  -i, --ignore FOLDER     Ignore the specified folder");
    println!("  -of, --only-folders     Show only folders");
    println!("  -l, --lines             Show the number of lines in files");
    println!("  -e, --extension EXT     Show only files with the specified extension");
    println!("  -d, --depth DEPTH       Limit the display depth");
    println!("  -o, --output FILE       Save output to file");
    println!("  -sc, --show-code        Show code content from all files");
    println!("  -a, --analyze           Analyze code and show statistics");
    println!("  -h, --help              Show this help message");
    println!();
    println!("Examples:");
    println!("  ssp                     Display the structure of the current directory");
    println!("  ssp /path/to/dir        Display the structure of the specified directory");
    println!("  ssp -i target -of       Only folders, ignore 'target'");
    println!("  ssp -l -e rs            .rs files with line counts");
    println!("  ssp -d 2                Show structure up to 2 levels deep");
    println!("  ssp -o output.txt       Save structure to file");
    println!("  ssp -sc                 Show all code content");
    println!("  ssp -a                  Analyze code statistics");
    println!("  ssp -sc -a -o full.txt  Full output with code and analysis to file");
}

fn display_structure(
    config: &Config,
    output: &mut String,
    code_files: &mut Vec<(PathBuf, String)>,
    stats: &mut CodeStats,
) {
    let root_name = config
        .target_dir
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(".");

    output.push_str(&format!("{}/\n", root_name));

    let mut entries = match read_dir_entries(&config.target_dir) {
        Some(e) => e,
        None => {
            eprintln!(
                "Error: Cannot read directory '{}'",
                config.target_dir.display()
            );
            return;
        }
    };

    filter_and_sort_entries(&mut entries, config);

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let path = entry.path();
        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            print_dir_structure(&path, "", is_last, 0, config, output, code_files, stats);
        } else {
            print_file_structure(&path, "", is_last, config, output, code_files, stats);
        }
    }
}

/// Reads directory entries, returning None on error instead of panicking.
fn read_dir_entries(path: &Path) -> Option<Vec<std::fs::DirEntry>> {
    let iter = fs::read_dir(path).ok()?;
    Some(iter.filter_map(|res| res.ok()).collect())
}

fn filter_and_sort_entries(entries: &mut Vec<std::fs::DirEntry>, config: &Config) {
    entries.retain(|entry| {
        // Use cached file_type() instead of calling path().is_dir() (avoids syscall)
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
        if is_dir {
            let name = entry.file_name();
            let name_str = name.to_str().unwrap_or("");
            !config.ignore_folders.contains(name_str)
        } else {
            if config.only_folders {
                return false;
            }
            if let Some(ref ext) = config.only_extension {
                entry
                    .path()
                    .extension()
                    .and_then(|e| e.to_str())
                    .is_some_and(|e| e == ext)
            } else {
                true
            }
        }
    });

    entries.sort_by(|a, b| {
        let a_is_dir = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
        let b_is_dir = b.file_type().map(|t| t.is_dir()).unwrap_or(false);
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });
}

#[allow(clippy::too_many_arguments)]
fn print_dir_structure(
    path: &Path,
    prefix: &str,
    is_last: bool,
    current_depth: usize,
    config: &Config,
    output: &mut String,
    code_files: &mut Vec<(PathBuf, String)>,
    stats: &mut CodeStats,
) {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("<invalid>");
    let connector = if is_last { "└──" } else { "├──" };
    let new_prefix_segment = if is_last { "    " } else { "│   " };

    output.push_str(&format!("{}{} {}\n", prefix, connector, name));

    if config.max_depth.is_some_and(|max| current_depth >= max) {
        return;
    }

    let new_prefix = format!("{}{}", prefix, new_prefix_segment);

    let mut entries = match read_dir_entries(path) {
        Some(e) => e,
        None => {
            eprintln!("Warning: Cannot read directory '{}'", path.display());
            return;
        }
    };

    filter_and_sort_entries(&mut entries, config);

    for (i, entry) in entries.iter().enumerate() {
        let is_last_entry = i == entries.len() - 1;
        let entry_path = entry.path();
        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            print_dir_structure(
                &entry_path,
                &new_prefix,
                is_last_entry,
                current_depth + 1,
                config,
                output,
                code_files,
                stats,
            );
        } else {
            print_file_structure(
                &entry_path,
                &new_prefix,
                is_last_entry,
                config,
                output,
                code_files,
                stats,
            );
        }
    }
}

fn print_file_structure(
    path: &Path,
    prefix: &str,
    is_last: bool,
    config: &Config,
    output: &mut String,
    code_files: &mut Vec<(PathBuf, String)>,
    stats: &mut CodeStats,
) {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("<invalid>");
    let connector = if is_last { "└──" } else { "├──" };

    // Read file content once for all purposes that need it
    let needs_content = config.show_lines || config.show_code || config.analyze_code;
    let content = if needs_content {
        fs::read_to_string(path).ok()
    } else {
        None
    };

    if config.show_lines {
        let line_count = content.as_deref().map_or(0, |c| c.lines().count());
        output.push_str(&format!("{}{} {} ({})\n", prefix, connector, name, line_count));
    } else {
        output.push_str(&format!("{}{} {}\n", prefix, connector, name));
    }

    if let Some(text) = content {
        if config.analyze_code {
            analyze_file(path, &text, stats);
        }
        if config.show_code {
            code_files.push((path.to_path_buf(), text));
        }
    }
}

fn analyze_file(path: &Path, content: &str, stats: &mut CodeStats) {
    stats.total_files += 1;

    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    *stats.files_by_extension.entry(ext.clone()).or_insert(0) += 1;

    let mut line_count = 0;
    for line in content.lines() {
        line_count += 1;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            stats.blank_lines += 1;
            continue;
        }

        if trimmed.starts_with("//")
            || trimmed.starts_with('#')
            || trimmed.starts_with("/*")
            || trimmed.starts_with('*')
        {
            stats.comment_lines += 1;
        }

        if trimmed.contains("int ")
            || trimmed.contains(": i32")
            || trimmed.contains(": i64")
            || trimmed.contains(": usize")
        {
            stats.int_count += 1;
        }
        if trimmed.contains("float ")
            || trimmed.contains("double ")
            || trimmed.contains(": f32")
            || trimmed.contains(": f64")
        {
            stats.float_count += 1;
        }
        if trimmed.contains("String")
            || trimmed.contains("str")
            || trimmed.contains("string")
            || trimmed.contains("&str")
        {
            stats.string_count += 1;
        }
        if trimmed.contains("bool") || trimmed.contains("boolean") {
            stats.bool_count += 1;
        }

        if trimmed.starts_with("fn ")
            || trimmed.starts_with("def ")
            || trimmed.starts_with("function ")
            || trimmed.contains("func ")
            || (trimmed.contains('(') && trimmed.contains(')') && trimmed.contains('{'))
        {
            stats.function_count += 1;
        }

        if trimmed.starts_with("class ")
            || trimmed.starts_with("struct ")
            || trimmed.starts_with("impl ")
            || trimmed.starts_with("trait ")
        {
            stats.class_count += 1;
        }
    }

    stats.total_lines += line_count;
    *stats.lines_by_extension.entry(ext).or_insert(0) += line_count;
}

fn format_analysis(stats: &CodeStats) -> String {
    let mut result = String::new();

    let code_lines = stats
        .total_lines
        .saturating_sub(stats.blank_lines + stats.comment_lines);

    result.push_str(&format!("Total Files: {}\n", stats.total_files));
    result.push_str(&format!("Total Lines: {}\n", stats.total_lines));
    result.push_str(&format!("Blank Lines: {}\n", stats.blank_lines));
    result.push_str(&format!("Comment Lines: {}\n", stats.comment_lines));
    result.push_str(&format!("Code Lines: {}\n\n", code_lines));

    result.push_str("Files by Extension:\n");
    let mut ext_vec: Vec<_> = stats.files_by_extension.iter().collect();
    ext_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (ext, count) in ext_vec {
        result.push_str(&format!("  .{}: {} files\n", ext, count));
    }

    result.push_str("\nLines by Extension:\n");
    let mut lines_vec: Vec<_> = stats.lines_by_extension.iter().collect();
    lines_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (ext, count) in lines_vec {
        result.push_str(&format!("  .{}: {} lines\n", ext, count));
    }

    result.push_str("\nCode Elements (approximate):\n");
    result.push_str(&format!("  Functions: {}\n", stats.function_count));
    result.push_str(&format!("  Classes/Structs: {}\n", stats.class_count));
    result.push_str(&format!("  Int declarations: {}\n", stats.int_count));
    result.push_str(&format!("  Float declarations: {}\n", stats.float_count));
    result.push_str(&format!("  String declarations: {}\n", stats.string_count));
    result.push_str(&format!("  Bool declarations: {}\n", stats.bool_count));

    if stats.total_lines > 0 {
        let code_percentage = (code_lines as f64 / stats.total_lines as f64) * 100.0;
        result.push_str(&format!("\nCode Density: {:.1}%\n", code_percentage));
    }

    result
}
