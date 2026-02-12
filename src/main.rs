use std::collections::{HashSet, HashMap};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;

// ============================================
// Configuration Structures
// ============================================

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
    show_hidden: bool,
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
            show_hidden: false,
        }
    }

    fn add_default_ignores(&mut self) {
        let defaults = vec![
            ".git", "node_modules", "__pycache__", "target", 
            ".idea", ".vscode", "dist", "build", ".DS_Store",
            "venv", ".env", "coverage", ".next", ".cache"
        ];
        
        for ignore in defaults {
            if !self.ignore_folders.contains(ignore) {
                self.ignore_folders.insert(ignore.to_string());
            }
        }
    }
}

// ============================================
// Code Statistics Structure
// ============================================

#[derive(Debug, Default)]
struct CodeStats {
    total_lines: usize,
    total_files: usize,
    files_by_extension: HashMap<String, usize>,
    lines_by_extension: HashMap<String, usize>,
    // Data type statistics
    int_count: usize,
    float_count: usize,
    string_count: usize,
    bool_count: usize,
    // Code structure statistics
    function_count: usize,
    class_count: usize,
    comment_lines: usize,
    blank_lines: usize,
    // File size statistics
    total_size_bytes: u64,
}

impl CodeStats {
    fn code_lines(&self) -> usize {
        self.total_lines.saturating_sub(self.blank_lines + self.comment_lines)
    }

    fn code_density(&self) -> f64 {
        if self.total_lines == 0 {
            0.0
        } else {
            (self.code_lines() as f64 / self.total_lines as f64) * 100.0
        }
    }
}

// ============================================
// Main Entry Point
// ============================================

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = match parse_arguments(args) {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Error: {}", err);
            eprintln!("\nUse --help for usage information");
            process::exit(1);
        }
    };

    let mut output = String::new();
    let mut code_files: Vec<(PathBuf, String)> = Vec::new();
    let mut stats = CodeStats::default();

    // Display directory structure
    if let Err(e) = display_structure(&config, &mut output, &mut code_files, &mut stats) {
        eprintln!("Error displaying structure: {}", e);
        process::exit(1);
    }

    // Append code content if requested
    if config.show_code && !code_files.is_empty() {
        append_code_content(&mut output, &code_files, &config);
    }

    // Append code analysis if requested
    if config.analyze_code {
        output.push_str("\n\n");
        output.push_str(&format_analysis(&stats));
    }

    // Output results
    if let Err(e) = write_output(&config, &output) {
        eprintln!("Error writing output: {}", e);
        process::exit(1);
    }
}

// ============================================
// Argument Parsing
// ============================================

fn parse_arguments(args: Vec<String>) -> Result<Config, String> {
    let mut config = Config::new();
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--ignore" | "-i" => {
                i += 1;
                if i >= args.len() {
                    return Err("--ignore flag requires an argument".to_string());
                }
                config.ignore_folders.insert(args[i].clone());
                i += 1;
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
                i += 1;
                if i >= args.len() {
                    return Err("--extension flag requires an argument".to_string());
                }
                config.only_extension = Some(args[i].clone());
                i += 1;
            }
            "--depth" | "-d" => {
                i += 1;
                if i >= args.len() {
                    return Err("--depth flag requires an argument".to_string());
                }
                match args[i].parse::<usize>() {
                    Ok(depth) if depth > 0 => {
                        config.max_depth = Some(depth);
                        i += 1;
                    }
                    _ => return Err("--depth requires a positive number".to_string()),
                }
            }
            "--output" | "-o" => {
                i += 1;
                if i >= args.len() {
                    return Err("--output flag requires an argument".to_string());
                }
                config.output_file = Some(args[i].clone());
                i += 1;
            }
            "--show-code" | "-sc" => {
                config.show_code = true;
                i += 1;
            }
            "--analyze" | "-a" => {
                config.analyze_code = true;
                i += 1;
            }
            "--show-hidden" | "-sh" => {
                config.show_hidden = true;
                i += 1;
            }
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            "--version" | "-v" => {
                print_version();
                process::exit(0);
            }
            arg if !arg.starts_with('-') => {
                let path = Path::new(arg);
                if !path.exists() {
                    return Err(format!("Path '{}' does not exist", arg));
                }
                if !path.is_dir() {
                    return Err(format!("'{}' is not a directory", arg));
                }
                config.target_dir = path.to_path_buf();
                i += 1;
            }
            unknown => {
                return Err(format!("Unknown flag: {}", unknown));
            }
        }
    }

    config.add_default_ignores();
    Ok(config)
}

// ============================================
// Help & Version
// ============================================

fn print_help() {
    println!("{}", include_str!("help_text.txt").trim_end());
}

fn print_version() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("SSP (Show Structure of Project) v{}", VERSION);
    println!("A powerful CLI tool for visualizing directory structures");
    println!("\nProject: https://github.com/Flaykky/show-struct-of-folder");
}

// Note: Create help_text.txt separately for better organization
// For now, using inline help:
fn print_help() {
    println!("SSP - Show Structure of Project v0.2.0");
    println!();
    println!("USAGE:");
    println!("    ssp [OPTIONS] [DIRECTORY]");
    println!();
    println!("ARGS:");
    println!("    <DIRECTORY>    Target directory (default: current directory)");
    println!();
    println!("OPTIONS:");
    println!("    -i, --ignore <FOLDER>      Ignore specified folder (can be used multiple times)");
    println!("    -of, --only-folders        Show only directories, hide files");
    println!("    -l, --lines                Show line count for each file");
    println!("    -e, --extension <EXT>      Filter by file extension (e.g., rs, py, js)");
    println!("    -d, --depth <NUM>          Limit tree depth (must be positive)");
    println!("    -o, --output <FILE>        Save output to file");
    println!("    -sc, --show-code           Extract and show code from all files");
    println!("    -a, --analyze              Analyze code and show statistics");
    println!("    -sh, --show-hidden         Show hidden files and folders");
    println!("    -h, --help                 Print this help message");
    println!("    -v, --version              Print version information");
    println!();
    println!("EXAMPLES:");
    println!("    ssp                        # Current directory structure");
    println!("    ssp /path/to/project       # Specific directory");
    println!("    ssp -l                     # With line counts");
    println!("    ssp -e rs -a               # Rust files with analysis");
    println!("    ssp -d 3 -of               # 3 levels deep, folders only");
    println!("    ssp -i node_modules -i dist  # Ignore multiple folders");
    println!("    ssp -sc -a -o report.txt   # Full report to file");
    println!();
    println!("DEFAULT IGNORED FOLDERS:");
    println!("    .git, node_modules, __pycache__, target, .idea, .vscode,");
    println!("    dist, build, .DS_Store, venv, .env, coverage, .next, .cache");
    println!();
    println!("For more information: https://github.com/Flaykky/show-struct-of-folder");
}

// ============================================
// Directory Structure Display
// ============================================

fn display_structure(
    config: &Config, 
    output: &mut String, 
    code_files: &mut Vec<(PathBuf, String)>,
    stats: &mut CodeStats
) -> io::Result<()> {
    let root_name = config.target_dir.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(".");

    output.push_str(&format!("{}/\n", root_name));

    let entries = read_and_filter_entries(&config.target_dir, config)?;

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let path = entry.path();
        
        if path.is_dir() {
            print_dir_structure(&path, "", is_last, 0, config, output, code_files, stats)?;
        } else {
            print_file_structure(&path, "", is_last, config, output, code_files, stats)?;
        }
    }

    Ok(())
}

fn read_and_filter_entries(dir: &Path, config: &Config) -> io::Result<Vec<fs::DirEntry>> {
    let mut entries: Vec<_> = fs::read_dir(dir)?
        .filter_map(|res| res.ok())
        .collect();

    filter_and_sort_entries(&mut entries, config);
    Ok(entries)
}

fn filter_and_sort_entries(entries: &mut Vec<fs::DirEntry>, config: &Config) {
    entries.retain(|entry| {
        let path = entry.path();
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        // Skip hidden files/folders if not requested
        if !config.show_hidden && name.starts_with('.') {
            return false;
        }

        if path.is_dir() {
            !config.ignore_folders.contains(name)
        } else {
            if config.only_folders {
                return false;
            }
            
            if let Some(ref ext) = config.only_extension {
                if let Some(file_ext) = path.extension() {
                    file_ext.to_str() == Some(ext)
                } else {
                    false
                }
            } else {
                true
            }
        }
    });

    // Sort: directories first, then files, alphabetically within each group
    entries.sort_by(|a, b| {
        let a_is_dir = a.path().is_dir();
        let b_is_dir = b.path().is_dir();
        
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => {
                let a_name = a.file_name().to_string_lossy().to_lowercase();
                let b_name = b.file_name().to_string_lossy().to_lowercase();
                a_name.cmp(&b_name)
            }
        }
    });
}

fn print_dir_structure(
    path: &Path, 
    prefix: &str, 
    is_last: bool, 
    current_depth: usize,
    config: &Config,
    output: &mut String,
    code_files: &mut Vec<(PathBuf, String)>,
    stats: &mut CodeStats
) -> io::Result<()> {
    let name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("?");
    
    let connector = if is_last { "└──" } else { "├──" };
    let new_prefix_segment = if is_last { "    " } else { "│   " };

    output.push_str(&format!("{}{} {}\n", prefix, connector, name));

    // Check depth limit
    if let Some(max_depth) = config.max_depth {
        if current_depth >= max_depth {
            return Ok(());
        }
    }

    let new_prefix = format!("{}{}", prefix, new_prefix_segment);
    let entries = read_and_filter_entries(path, config)?;

    for (i, entry) in entries.iter().enumerate() {
        let is_last_entry = i == entries.len() - 1;
        let entry_path = entry.path();
        
        if entry_path.is_dir() {
            print_dir_structure(
                &entry_path, 
                &new_prefix, 
                is_last_entry, 
                current_depth + 1, 
                config, 
                output, 
                code_files, 
                stats
            )?;
        } else {
            print_file_structure(
                &entry_path, 
                &new_prefix, 
                is_last_entry, 
                config, 
                output, 
                code_files, 
                stats
            )?;
        }
    }

    Ok(())
}

fn print_file_structure(
    path: &Path, 
    prefix: &str, 
    is_last: bool, 
    config: &Config,
    output: &mut String,
    code_files: &mut Vec<(PathBuf, String)>,
    stats: &mut CodeStats
) -> io::Result<()> {
    let name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("?");
    
    let connector = if is_last { "└──" } else { "├──" };
    
    let file_info = if config.show_lines {
        let line_count = count_lines(path)?;
        let size = format_file_size(path);
        format!("{}{} {} ({} lines, {})", prefix, connector, name, line_count, size)
    } else {
        format!("{}{} {}", prefix, connector, name)
    };
    
    output.push_str(&file_info);
    output.push('\n');

    // Collect code and/or analyze if needed
    if config.show_code || config.analyze_code {
        if let Ok(content) = fs::read_to_string(path) {
            if config.show_code {
                code_files.push((path.to_path_buf(), content.clone()));
            }
            if config.analyze_code {
                analyze_file(path, &content, stats)?;
            }
        }
    }

    Ok(())
}

// ============================================
// File Analysis Functions
// ============================================

fn count_lines(path: &Path) -> io::Result<usize> {
    let content = fs::read_to_string(path)?;
    Ok(content.lines().count())
}

fn format_file_size(path: &Path) -> String {
    if let Ok(metadata) = fs::metadata(path) {
        let size = metadata.len();
        if size < 1024 {
            format!("{} B", size)
        } else if size < 1024 * 1024 {
            format!("{:.1} KB", size as f64 / 1024.0)
        } else {
            format!("{:.2} MB", size as f64 / (1024.0 * 1024.0))
        }
    } else {
        "? B".to_string()
    }
}

fn analyze_file(path: &Path, content: &str, stats: &mut CodeStats) -> io::Result<()> {
    stats.total_files += 1;
    
    if let Ok(metadata) = fs::metadata(path) {
        stats.total_size_bytes += metadata.len();
    }
    
    let ext = path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    *stats.files_by_extension.entry(ext.clone()).or_insert(0) += 1;
    
    let lines: Vec<&str> = content.lines().collect();
    let line_count = lines.len();
    
    stats.total_lines += line_count;
    *stats.lines_by_extension.entry(ext.clone()).or_insert(0) += line_count;
    
    // Analyze each line
    for line in &lines {
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            stats.blank_lines += 1;
            continue;
        }
        
        // Comments (simplified detection)
        if is_comment_line(trimmed, &ext) {
            stats.comment_lines += 1;
        }
        
        // Data types
        count_data_types(trimmed, stats);
        
        // Functions and classes
        count_code_structures(trimmed, stats);
    }

    Ok(())
}

fn is_comment_line(line: &str, ext: &str) -> bool {
    match ext {
        "rs" | "c" | "cpp" | "java" | "js" | "ts" | "go" | "swift" => {
            line.starts_with("//") || line.starts_with("/*") || line.starts_with("*")
        }
        "py" | "rb" | "sh" | "yaml" | "yml" => {
            line.starts_with("#")
        }
        "html" | "xml" => {
            line.starts_with("<!--")
        }
        _ => {
            line.starts_with("//") || line.starts_with("#") || 
            line.starts_with("/*") || line.starts_with("*")
        }
    }
}

fn count_data_types(line: &str, stats: &mut CodeStats) {
    let lower = line.to_lowercase();
    
    // Integer types
    if lower.contains("int ") || lower.contains(": i32") || 
       lower.contains(": i64") || lower.contains(": usize") ||
       lower.contains(": u32") || lower.contains(": u64") {
        stats.int_count += 1;
    }
    
    // Float types
    if lower.contains("float ") || lower.contains("double ") || 
       lower.contains(": f32") || lower.contains(": f64") {
        stats.float_count += 1;
    }
    
    // String types
    if lower.contains("string") || lower.contains("str") || 
       lower.contains("&str") || lower.contains("char") {
        stats.string_count += 1;
    }
    
    // Boolean types
    if lower.contains("bool") || lower.contains("boolean") {
        stats.bool_count += 1;
    }
}

fn count_code_structures(line: &str, stats: &mut CodeStats) {
    let trimmed = line.trim();
    
    // Functions
    if trimmed.starts_with("fn ") || 
       trimmed.starts_with("def ") || 
       trimmed.starts_with("function ") || 
       trimmed.starts_with("func ") ||
       (trimmed.contains("(") && trimmed.contains(")") && trimmed.contains("{") &&
        !trimmed.starts_with("if") && !trimmed.starts_with("while") && !trimmed.starts_with("for")) {
        stats.function_count += 1;
    }
    
    // Classes and structures
    if trimmed.starts_with("class ") || 
       trimmed.starts_with("struct ") || 
       trimmed.starts_with("impl ") || 
       trimmed.starts_with("trait ") ||
       trimmed.starts_with("interface ") ||
       trimmed.starts_with("enum ") {
        stats.class_count += 1;
    }
}

// ============================================
// Output Formatting
// ============================================

fn append_code_content(
    output: &mut String, 
    code_files: &[(PathBuf, String)],
    config: &Config
) {
    output.push_str("\n\n");
    output.push_str("═══════════════════════════════════════════════════════════════════════════════\n");
    output.push_str("                              CODE CONTENT                                    \n");
    output.push_str("═══════════════════════════════════════════════════════════════════════════════\n\n");
    
    for (idx, (path, content)) in code_files.iter().enumerate() {
        let relative_path = path.strip_prefix(&config.target_dir)
            .unwrap_or(path)
            .to_str()
            .unwrap_or("?");
        
        output.push_str(&format!("┌─ File {}/{}: {}\n", idx + 1, code_files.len(), relative_path));
        output.push_str("│\n");
        
        for (line_num, line) in content.lines().enumerate() {
            output.push_str(&format!("│ {:4} │ {}\n", line_num + 1, line));
        }
        
        output.push_str("└");
        output.push_str(&"─".repeat(78));
        output.push_str("\n\n");
    }
}

fn format_analysis(stats: &CodeStats) -> String {
    let mut result = String::new();
    
    result.push_str("═══════════════════════════════════════════════════════════════════════════════\n");
    result.push_str("                            CODE ANALYSIS                                     \n");
    result.push_str("═══════════════════════════════════════════════════════════════════════════════\n\n");
    
    // Overview
    result.push_str("OVERVIEW:\n");
    result.push_str(&format!("  Total Files:        {:>10}\n", format_number(stats.total_files)));
    result.push_str(&format!("  Total Lines:        {:>10}\n", format_number(stats.total_lines)));
    result.push_str(&format!("  Total Size:         {:>10}\n", format_bytes(stats.total_size_bytes)));
    result.push_str(&format!("  Blank Lines:        {:>10}\n", format_number(stats.blank_lines)));
    result.push_str(&format!("  Comment Lines:      {:>10}\n", format_number(stats.comment_lines)));
    result.push_str(&format!("  Code Lines:         {:>10}\n", format_number(stats.code_lines())));
    result.push_str(&format!("  Code Density:       {:>10.1}%\n", stats.code_density()));
    
    // Files by extension
    result.push_str("\nFILES BY EXTENSION:\n");
    let mut ext_vec: Vec<_> = stats.files_by_extension.iter().collect();
    ext_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (ext, count) in ext_vec.iter().take(10) {
        let percentage = (*count as f64 / stats.total_files as f64) * 100.0;
        result.push_str(&format!("  .{:<12} {:>6} files  ({:>5.1}%)\n", ext, count, percentage));
    }
    
    // Lines by extension
    result.push_str("\nLINES BY EXTENSION:\n");
    let mut lines_vec: Vec<_> = stats.lines_by_extension.iter().collect();
    lines_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (ext, count) in lines_vec.iter().take(10) {
        let percentage = (*count as f64 / stats.total_lines as f64) * 100.0;
        result.push_str(&format!("  .{:<12} {:>6} lines  ({:>5.1}%)\n", ext, format_number(**count), percentage));
    }
    
    // Code elements
    result.push_str("\nCODE ELEMENTS (approximate):\n");
    result.push_str(&format!("  Functions:          {:>10}\n", format_number(stats.function_count)));
    result.push_str(&format!("  Classes/Structs:    {:>10}\n", format_number(stats.class_count)));
    result.push_str(&format!("  Int declarations:   {:>10}\n", format_number(stats.int_count)));
    result.push_str(&format!("  Float declarations: {:>10}\n", format_number(stats.float_count)));
    result.push_str(&format!("  String declarations:{:>10}\n", format_number(stats.string_count)));
    result.push_str(&format!("  Bool declarations:  {:>10}\n", format_number(stats.bool_count)));
    
    result.push_str("\n");
    result.push_str("═══════════════════════════════════════════════════════════════════════════════\n");
    
    result
}

fn format_number(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

// ============================================
// Output Writing
// ============================================

fn write_output(config: &Config, output: &str) -> io::Result<()> {
    if let Some(filename) = &config.output_file {
        let mut file = fs::File::create(filename)?;
        file.write_all(output.as_bytes())?;
        println!("✓ Output saved to: {}", filename);
        println!("  {} bytes written", output.len());
    } else {
        print!("{}", output);
    }
    Ok(())
}
