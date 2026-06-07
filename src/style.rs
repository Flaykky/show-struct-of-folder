//! Color and icon resolution, theme application, tty detection.

use std::io::IsTerminal;
use owo_colors::{OwoColorize, Style};

use crate::cli::{ColorWhen, IconWhen};
use crate::config::Theme;
use crate::icons::{self, DIR_ICON, SYMLINK_ICON};
use crate::tree::NodeKind;

/// Resolved at startup; passed everywhere so we don't re-query `isatty`.
#[derive(Debug, Clone, Copy)]
pub struct StyleConfig {
    pub use_color: bool,
    pub use_icons: bool,
    pub ascii: bool,
}

impl StyleConfig {
    pub fn resolve(color: ColorWhen, icons: IconWhen, no_icons: bool, ascii: bool) -> Self {
        let tty = std::io::stdout().is_terminal();
        let no_color_env = std::env::var_os("NO_COLOR").is_some();

        let use_color = match color {
            ColorWhen::Always => true,
            ColorWhen::Never => false,
            ColorWhen::Auto => tty && !no_color_env,
        };

        let use_icons = if no_icons {
            false
        } else {
            match icons {
                IconWhen::Always => true,
                IconWhen::Never => false,
                // Auto: only show icons when color is also on (implies a rich terminal)
                IconWhen::Auto => use_color,
            }
        };

        Self { use_color, use_icons, ascii }
    }

    /// Tree connector characters.
    pub fn branch_mid(&self) -> &'static str {
        if self.ascii { "|-- " } else { "├── " }
    }
    pub fn branch_last(&self) -> &'static str {
        if self.ascii { "`-- " } else { "└── " }
    }
    pub fn branch_pipe(&self) -> &'static str {
        if self.ascii { "|   " } else { "│   " }
    }
    pub fn branch_blank(&self) -> &'static str {
        "    "
    }
}

/// Color palette derived from a Theme.  Stores pre-built `owo_colors::Style`.
/// Fields for exec/media/archive/config/doc exist for future per-type coloring.
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct Palette {
    pub dir: Style,
    pub file: Style,
    pub symlink: Style,
    pub exec: Style,
    pub image: Style,
    pub audio: Style,
    pub video: Style,
    pub archive: Style,
    pub config: Style,
    pub doc: Style,
    pub git_modified: Style,
    pub git_untracked: Style,
    pub git_staged: Style,
    pub git_ignored: Style,
    pub meta: Style,           // line-counts, sizes
    pub connector: Style,      // tree branches
    pub count: Style,          // summary counts
}

impl Palette {
    /// Build from a theme; if `use_color` is false, return an all-default (no-op) palette.
    pub fn from_theme(theme: &Theme, use_color: bool) -> Self {
        if !use_color {
            return Self::default();
        }

        let colors = &theme.colors;
        Self {
            dir:          parse_style(&colors.dir),
            file:         parse_style(&colors.file),
            symlink:      parse_style(&colors.symlink),
            exec:         parse_style(&colors.exec),
            image:        parse_style(&colors.image),
            audio:        parse_style(&colors.audio),
            video:        parse_style(&colors.video),
            archive:      parse_style(&colors.archive),
            config:       parse_style(&colors.config_file),
            doc:          parse_style(&colors.doc),
            git_modified: parse_style(&colors.git_modified),
            git_untracked:parse_style(&colors.git_untracked),
            git_staged:   parse_style(&colors.git_staged),
            git_ignored:  parse_style(&colors.git_ignored),
            meta:         parse_style(&colors.meta),
            connector:    parse_style(&colors.connector),
            count:        parse_style(&colors.count),
        }
    }
}

fn parse_style(s: &str) -> Style {
    match s {
        "black"         => Style::new().black(),
        "red"           => Style::new().red(),
        "green"         => Style::new().green(),
        "yellow"        => Style::new().yellow(),
        "blue"          => Style::new().blue(),
        "magenta"       => Style::new().magenta(),
        "cyan"          => Style::new().cyan(),
        "white"         => Style::new().white(),
        "bright_black" | "dark_gray" => Style::new().bright_black(),
        "bright_red"    => Style::new().bright_red(),
        "bright_green"  => Style::new().bright_green(),
        "bright_yellow" => Style::new().bright_yellow(),
        "bright_blue"   => Style::new().bright_blue(),
        "bright_magenta"=> Style::new().bright_magenta(),
        "bright_cyan"   => Style::new().bright_cyan(),
        "bright_white"  => Style::new().bright_white(),
        "bold"          => Style::new().bold(),
        _               => Style::new(),
    }
}

/// Returns the right icon string for an entry.
pub fn icon_str(name: &str, kind: NodeKind, style_cfg: &StyleConfig) -> &'static str {
    if !style_cfg.use_icons {
        return "";
    }
    // Name overrides extension
    if let Some(icon) = icons::icon_for_name(&name.to_lowercase()) {
        return icon;
    }
    match kind {
        NodeKind::Dir => DIR_ICON,
        NodeKind::Symlink => SYMLINK_ICON,
        NodeKind::File => {
            let ext = name.rfind('.').map(|i| &name[i + 1..]).unwrap_or("").to_lowercase();
            icons::icon_for_ext(&ext)
        }
    }
}

/// Return colored string for a name using the palette based on kind.
pub fn paint_name(s: &str, kind: NodeKind, palette: &Palette) -> String {
    match kind {
        NodeKind::Dir => format!("{}", s.style(palette.dir)),
        NodeKind::Symlink => format!("{}", s.style(palette.symlink)),
        NodeKind::File => format!("{}", s.style(palette.file)),
    }
}

/// Paint a connector segment.
pub fn paint_connector(s: &str, palette: &Palette) -> String {
    format!("{}", s.style(palette.connector))
}

/// Paint metadata (lines, size).
pub fn paint_meta(s: &str, palette: &Palette) -> String {
    format!("{}", s.style(palette.meta))
}

/// Map a git status byte to a colored glyph string.
pub fn git_status_glyph(xy: [u8; 2], palette: &Palette) -> String {
    // Porcelain v1: XY where X=index, Y=worktree
    let index = xy[0];
    let work = xy[1];
    if index == b'?' && work == b'?' {
        return format!("{}", "?".style(palette.git_untracked));
    }
    if index == b'!' && work == b'!' {
        return format!("{}", "!".style(palette.git_ignored));
    }
    // Staged changes
    if matches!(index, b'A' | b'M' | b'R' | b'C' | b'D') {
        return format!("{}", "●".style(palette.git_staged));
    }
    // Worktree modifications
    if matches!(work, b'M' | b'D') {
        return format!("{}", "✚".style(palette.git_modified));
    }
    String::new()
}
