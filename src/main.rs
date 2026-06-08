mod analyze;
mod cli;
mod config;
mod git;
mod icons;
mod output;
mod render;
mod style;
mod tree;
mod walk;

use std::collections::HashSet;
use std::io::Write as _;
use std::path::PathBuf;

use clap::Parser;

use cli::{Args, ColorWhen, IconWhen, OutputFormat, SortKey};
use config::{ConfigDefaults, ConfigFile, Theme};
use output::{Summary, to_flat_list, to_json, to_markdown};
use render::{RenderContext, RenderOptions, render_tree};
use style::{Palette, StyleConfig};
use walk::{WalkOptions, build_tree};

// Windows: enable virtual terminal processing so ANSI codes work on older consoles.
#[cfg(windows)]
fn enable_ansi_windows() {
    use std::os::windows::io::AsRawHandle;
    extern "system" {
        fn GetConsoleMode(handle: *mut std::ffi::c_void, mode: *mut u32) -> i32;
        fn SetConsoleMode(handle: *mut std::ffi::c_void, mode: u32) -> i32;
    }
    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;
    unsafe {
        let handle = std::io::stdout().as_raw_handle();
        let mut mode: u32 = 0;
        if GetConsoleMode(handle as _, &mut mode) != 0 {
            SetConsoleMode(handle as _, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
}

fn main() {
    #[cfg(windows)]
    enable_ansi_windows();

    let args = Args::parse();

    // ── --generate-config ────────────────────────────────────────────────────
    if args.generate_config {
        match ConfigFile::generate_default() {
            Ok(path) => {
                println!("Default config written to: {}", path.display());
            }
            Err(e) => {
                eprintln!("ssp: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    // ── Load config file ─────────────────────────────────────────────────────
    let config_file: Option<ConfigFile> = if args.no_config {
        None
    } else {
        ConfigFile::load(args.config.as_deref())
    };

    let cfg_defaults = config_file
        .as_ref()
        .map(|c| &c.defaults)
        .map(|d| d.clone())
        .unwrap_or_default();

    // ── Resolve active theme ──────────────────────────────────────────────────
    let theme_name = args
        .theme
        .as_deref()
        .unwrap_or(&cfg_defaults.theme)
        .to_string();

    let theme: Theme = config_file
        .as_ref()
        .map(|c| c.resolve_theme(&theme_name))
        .unwrap_or_default();

    // ── Style config (color / icons / ascii) ─────────────────────────────────
    let color_when = resolve_color(args.color, &cfg_defaults, args.no_config);
    let icon_when = resolve_icons(args.icons, args.no_icons, &cfg_defaults, args.no_config);

    let style_cfg = StyleConfig::resolve(color_when, icon_when, args.no_icons, args.ascii);
    let palette = Palette::from_theme(&theme, style_cfg.use_color);

    // ── Target directory ─────────────────────────────────────────────────────
    let target_dir: PathBuf = args
        .directory
        .as_deref()
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    if !target_dir.exists() {
        eprintln!("ssp: '{}' does not exist", target_dir.display());
        std::process::exit(1);
    }
    if !target_dir.is_dir() {
        eprintln!("ssp: '{}' is not a directory", target_dir.display());
        std::process::exit(1);
    }

    // ── Ignore set ────────────────────────────────────────────────────────────
    let mut ignore_names: HashSet<String> = HashSet::new();
    // Default ignores from config
    for name in &cfg_defaults.ignore {
        ignore_names.insert(name.clone());
    }
    // If no config loaded, use hardcoded defaults
    if config_file.is_none() {
        for name in [".git", "node_modules", "target", "__pycache__", ".idea", ".vscode"] {
            ignore_names.insert(name.to_string());
        }
    }
    // CLI --ignore flags
    for name in &args.ignore_names {
        ignore_names.insert(name.clone());
    }
    let dirs_only = args.dirs_only;
    // Back-compat -sc
    let show_code = args.show_code || args.sc_compat;

    // ── Sort key ─────────────────────────────────────────────────────────────
    let sort_key = resolve_sort(args.sort, &cfg_defaults);

    // ── Git status ────────────────────────────────────────────────────────────
    let git_status = if args.git {
        git::collect_status(&target_dir)
    } else {
        None
    };

    // ── Build tree ────────────────────────────────────────────────────────────
    let walk_opts = WalkOptions {
        root: &target_dir,
        max_depth: args.depth.or(cfg_defaults.depth),
        show_hidden: args.show_hidden || cfg_defaults.show_hidden,
        respect_gitignore: !args.no_gitignore,
        ignore_names: &ignore_names,
        include_globs: &args.include_globs,
        exclude_globs: &args.exclude_globs,
        extension_filter: args.extension.as_deref(),
        dirs_only,
        files_only: args.files_only,
        prune: args.prune,
        sort: sort_key,
        reverse: args.reverse,
        dirs_first: resolve_dirs_first(args.dirs_first, args.no_dirs_first, &cfg_defaults),
        git_status: git_status.as_ref(),
    };

    let root_node = build_tree(&target_dir, &walk_opts);

    // ── Format output ─────────────────────────────────────────────────────────
    let mut final_output = String::new();

    match args.format {
        OutputFormat::Json => {
            let val = to_json(&root_node, args.sizes);
            match serde_json::to_string_pretty(&val) {
                Ok(s) => final_output.push_str(&s),
                Err(e) => {
                    eprintln!("ssp: JSON serialization error: {}", e);
                    std::process::exit(1);
                }
            }
            final_output.push('\n');
        }
        OutputFormat::Markdown => {
            final_output.push_str(&to_markdown(&root_node, 0));
        }
        OutputFormat::List => {
            let paths = to_flat_list(&root_node, &target_dir);
            for p in paths {
                final_output.push_str(&p);
                final_output.push('\n');
            }
        }
        OutputFormat::Tree => {
            let render_opts = RenderOptions {
                show_lines: args.show_lines,
                show_sizes: args.sizes,
                show_git: args.git,
                show_code,
                analyze: args.analyze,
                full_path: args.full_path,
                root_dir: target_dir.clone(),
            };
            let render_ctx = RenderContext {
                style: &style_cfg,
                palette: &palette,
                opts: &render_opts,
            };

            let mut stats = analyze::CodeStats::default();
            let mut code_files: Vec<(std::path::PathBuf, String)> = Vec::new();

            let tree_str = render_tree(&root_node, &render_ctx, &mut stats, &mut code_files);
            final_output.push_str(&tree_str);

            // Summary
            if args.summary {
                let summary = Summary::from_node(&root_node);
                final_output.push_str(&summary.format());
                final_output.push('\n');
            }

            // Code content section
            if show_code && !code_files.is_empty() {
                final_output.push_str("\n\n=== CODE CONTENT ===\n\n");
                for (idx, (path, content)) in code_files.iter().enumerate() {
                    let rel = path
                        .strip_prefix(&target_dir)
                        .unwrap_or(path)
                        .to_string_lossy();
                    final_output.push_str(&format!("{}. {}:\n\n", idx + 1, rel));
                    final_output.push_str(content);
                    final_output.push_str("\n\n");
                    final_output.push_str(&"-".repeat(80));
                    final_output.push_str("\n\n");
                }
            }

            // Analysis section (render_tree already collected stats when analyze=true)
            if args.analyze {
                final_output.push_str("\n\n=== CODE ANALYSIS ===\n\n");
                final_output.push_str(&analyze::format_analysis(&stats));
            }
        }
    }

    // ── Write output ──────────────────────────────────────────────────────────
    if let Some(filename) = &args.output_file {
        match std::fs::File::create(filename) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(final_output.as_bytes()) {
                    eprintln!("ssp: error writing to '{}': {}", filename, e);
                    std::process::exit(1);
                }
                // Print confirmation to stderr so it doesn't pollute the file content
                eprintln!("Output saved to: {}", filename);
            }
            Err(e) => {
                eprintln!("ssp: cannot create '{}': {}", filename, e);
                std::process::exit(1);
            }
        }
    } else {
        print!("{}", final_output);
    }
}

fn resolve_color(cli_value: ColorWhen, defaults: &ConfigDefaults, no_config: bool) -> ColorWhen {
    if no_config || cli_value != ColorWhen::Auto {
        return cli_value;
    }

    match defaults.color.as_str() {
        "always" => ColorWhen::Always,
        "never" => ColorWhen::Never,
        _ => ColorWhen::Auto,
    }
}

fn resolve_icons(
    cli_value: IconWhen,
    no_icons: bool,
    defaults: &ConfigDefaults,
    no_config: bool,
) -> IconWhen {
    if no_icons {
        return IconWhen::Never;
    }
    if no_config || cli_value != IconWhen::Auto {
        return cli_value;
    }
    if defaults.icons {
        IconWhen::Auto
    } else {
        IconWhen::Never
    }
}

fn resolve_sort(cli_value: Option<SortKey>, defaults: &ConfigDefaults) -> SortKey {
    cli_value.unwrap_or_else(|| match defaults.sort.as_str() {
        "size" => SortKey::Size,
        "time" => SortKey::Time,
        "ext" => SortKey::Ext,
        "none" => SortKey::None,
        _ => SortKey::Name,
    })
}

fn resolve_dirs_first(
    cli_dirs_first: bool,
    cli_no_dirs_first: bool,
    defaults: &ConfigDefaults,
) -> bool {
    if cli_no_dirs_first {
        return false;
    }
    if cli_dirs_first {
        return true;
    }
    defaults.dirs_first
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_defaults_feed_runtime_options() {
        let defaults = ConfigDefaults {
            icons: false,
            color: "never".into(),
            sort: "size".into(),
            dirs_first: false,
            ..ConfigDefaults::default()
        };

        assert_eq!(resolve_color(ColorWhen::Auto, &defaults, false), ColorWhen::Never);
        assert_eq!(resolve_icons(IconWhen::Auto, false, &defaults, false), IconWhen::Never);
        assert_eq!(resolve_sort(None, &defaults), SortKey::Size);
        assert!(!resolve_dirs_first(false, false, &defaults));
        assert!(resolve_dirs_first(true, false, &defaults));
    }

    #[test]
    fn cli_flags_override_config_defaults() {
        let defaults = ConfigDefaults {
            icons: false,
            color: "never".into(),
            dirs_first: true,
            ..ConfigDefaults::default()
        };

        assert_eq!(resolve_color(ColorWhen::Always, &defaults, false), ColorWhen::Always);
        assert_eq!(resolve_icons(IconWhen::Always, false, &defaults, false), IconWhen::Always);
        assert!(!resolve_dirs_first(true, true, &defaults));
    }
}
