use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ColorWhen {
    Auto,
    Always,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum IconWhen {
    Auto,
    Always,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum SortKey {
    Name,
    Size,
    Time,
    Ext,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Tree,
    Json,
    Markdown,
    List,
}

#[derive(Parser, Debug)]
#[command(
    name = "ssp",
    version = "2.0.0",
    about = "SSP – Show Structure of Project\nA modern, configurable tree alternative",
    long_about = None,
)]
pub struct Args {
    /// Directory to display (default: current directory)
    pub directory: Option<String>,

    // ── Display ───────────────────────────────────────────────────────────────
    /// When to use ANSI colors
    #[arg(long, value_enum, default_value = "auto", value_name = "WHEN")]
    pub color: ColorWhen,

    /// When to show file-type icons (requires a Nerd Font)
    #[arg(long, value_enum, default_value = "auto", value_name = "WHEN")]
    pub icons: IconWhen,

    /// Disable icons (shorthand for --icons=never)
    #[arg(long, overrides_with = "icons")]
    pub no_icons: bool,

    /// Use ASCII connectors instead of Unicode box-drawing characters
    #[arg(long)]
    pub ascii: bool,

    /// Show the full path for each entry
    #[arg(short = 'f', long)]
    pub full_path: bool,

    // ── Depth ─────────────────────────────────────────────────────────────────
    /// Limit display depth (also -L)
    #[arg(short = 'd', long = "depth", visible_short_alias = 'L', value_name = "N")]
    pub depth: Option<usize>,

    // ── Filtering ─────────────────────────────────────────────────────────────
    /// Show hidden files and directories (starting with '.')
    #[arg(short = 'a', long = "all")]
    pub show_hidden: bool,

    /// Do not respect .gitignore rules (they are respected by default)
    #[arg(long)]
    pub no_gitignore: bool,

    /// Ignore a specific folder by name (repeatable; default list always applied)
    #[arg(short = 'i', long = "ignore", value_name = "NAME", action = clap::ArgAction::Append)]
    pub ignore_names: Vec<String>,

    /// Include only files matching this glob pattern (repeatable)
    #[arg(short = 'P', long = "pattern", value_name = "GLOB", action = clap::ArgAction::Append)]
    pub include_globs: Vec<String>,

    /// Exclude files/directories matching this glob pattern (repeatable)
    #[arg(short = 'I', long = "ignore-glob", value_name = "GLOB", action = clap::ArgAction::Append)]
    pub exclude_globs: Vec<String>,

    /// Show only files with this extension
    #[arg(short = 'e', long = "extension", value_name = "EXT")]
    pub extension: Option<String>,

    /// Show only directories (also: --only-folders for back-compat)
    #[arg(short = 'D', long = "dirs-only", alias = "only-folders")]
    pub dirs_only: bool,

    /// Show only files (no directories)
    #[arg(long)]
    pub files_only: bool,

    /// Remove empty directories from the output
    #[arg(long)]
    pub prune: bool,

    // ── Sorting ───────────────────────────────────────────────────────────────
    /// Sort entries by the given key
    #[arg(short = 's', long = "sort", value_enum, value_name = "KEY")]
    pub sort: Option<SortKey>,

    /// Reverse the sort order
    #[arg(short = 'r', long)]
    pub reverse: bool,

    /// List directories before files (default: true)
    #[arg(long, default_value_t = true, overrides_with = "no_dirs_first")]
    pub dirs_first: bool,

    #[arg(long, hide = true)]
    pub no_dirs_first: bool,

    // ── Git ───────────────────────────────────────────────────────────────────
    /// Show git status markers next to each entry
    #[arg(long)]
    pub git: bool,

    // ── Metadata / counts ─────────────────────────────────────────────────────
    /// Show line count next to each file
    #[arg(short = 'l', long = "lines")]
    pub show_lines: bool,

    /// Show file sizes next to each file
    #[arg(long)]
    pub sizes: bool,

    /// Print a summary line at the end (file count, dir count, total size)
    #[arg(long)]
    pub summary: bool,

    // ── Code analysis ─────────────────────────────────────────────────────────
    /// Show the content of all files after the tree
    #[arg(long = "show-code", visible_short_alias = 'S')]
    pub show_code: bool,

    /// Hidden alias kept for back-compat (-sc)
    #[arg(long = "sc", hide = true)]
    pub sc_compat: bool,

    /// Analyze code and show statistics
    #[arg(long = "analyze", short = 'A')]
    pub analyze: bool,

    /// Hidden alias kept for back-compat (-a in old version)
    // NOTE: -a is now taken by --all; old -a users should use -A
    // We keep --analyze as the canonical flag; there is no silent override.

    // ── Output ────────────────────────────────────────────────────────────────
    /// Save output to a file instead of stdout
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    pub output_file: Option<String>,

    /// Output format
    #[arg(long = "format", value_enum, default_value = "tree", value_name = "FMT")]
    pub format: OutputFormat,

    // ── Config / themes ───────────────────────────────────────────────────────
    /// Use a specific config file
    #[arg(long, value_name = "FILE")]
    pub config: Option<String>,

    /// Select a named theme from the config file
    #[arg(long, value_name = "NAME")]
    pub theme: Option<String>,

    /// Ignore the config file entirely
    #[arg(long)]
    pub no_config: bool,

    /// Write a default config file to the config directory and exit
    #[arg(long)]
    pub generate_config: bool,
}
