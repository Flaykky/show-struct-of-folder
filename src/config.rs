//! Config file loading, default theme, and CLI-merge logic.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// ── Colour palette for a theme ────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeColors {
    pub dir: String,
    pub file: String,
    pub symlink: String,
    pub exec: String,
    pub image: String,
    pub audio: String,
    pub video: String,
    pub archive: String,
    pub config_file: String,
    pub doc: String,
    pub git_modified: String,
    pub git_untracked: String,
    pub git_staged: String,
    pub git_ignored: String,
    pub meta: String,
    pub connector: String,
    pub count: String,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            dir: "blue".into(),
            file: "white".into(),
            symlink: "cyan".into(),
            exec: "green".into(),
            image: "magenta".into(),
            audio: "magenta".into(),
            video: "magenta".into(),
            archive: "red".into(),
            config_file: "yellow".into(),
            doc: "bright_white".into(),
            git_modified: "yellow".into(),
            git_untracked: "red".into(),
            git_staged: "green".into(),
            git_ignored: "bright_black".into(),
            meta: "bright_black".into(),
            connector: "bright_black".into(),
            count: "bright_white".into(),
        }
    }
}

// ── Icon overrides per theme ──────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThemeIcons(pub HashMap<String, String>);

// ── Theme bundle ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Theme {
    pub colors: ThemeColors,
    pub icons: ThemeIcons,
}

// ── Defaults section of the config file ──────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ConfigDefaults {
    pub icons: bool,
    pub color: String,
    pub sort: String,
    pub dirs_first: bool,
    pub ignore: Vec<String>,
    pub show_hidden: bool,
    pub depth: Option<usize>,
    pub theme: String,
}

impl Default for ConfigDefaults {
    fn default() -> Self {
        Self {
            icons: true,
            color: "auto".into(),
            sort: "name".into(),
            dirs_first: true,
            ignore: vec![
                ".git".into(),
                "node_modules".into(),
                "target".into(),
                "__pycache__".into(),
                ".idea".into(),
                ".vscode".into(),
            ],
            show_hidden: false,
            depth: None,
            theme: "default".into(),
        }
    }
}

// ── Top-level config file ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ConfigFile {
    pub defaults: ConfigDefaults,
    pub themes: HashMap<String, Theme>,
}

impl Default for ConfigFile {
    fn default() -> Self {
        let mut themes = HashMap::new();
        themes.insert("default".into(), Theme::default());
        Self {
            defaults: ConfigDefaults::default(),
            themes,
        }
    }
}

impl ConfigFile {
    /// Discover and parse the config file.  Returns `None` if none found.
    pub fn load(explicit_path: Option<&str>) -> Option<Self> {
        let path = if let Some(p) = explicit_path {
            PathBuf::from(p)
        } else if let Ok(p) = std::env::var("SSP_CONFIG") {
            PathBuf::from(p)
        } else {
            dirs::config_dir()?.join("ssp").join("config.toml")
        };

        let content = std::fs::read_to_string(&path).ok()?;
        match toml::from_str(&content) {
            Ok(cfg) => Some(cfg),
            Err(e) => {
                eprintln!("ssp: config parse error in {}: {}", path.display(), e);
                None
            }
        }
    }

    /// Resolve the active theme by name.
    pub fn resolve_theme(&self, name: &str) -> Theme {
        self.themes.get(name).cloned().unwrap_or_default()
    }

    /// Generate a default TOML config string.
    pub fn default_toml() -> String {
        r#"# SSP configuration file
# All values shown are the built-in defaults.

[defaults]
icons      = true         # show Nerd Font icons (auto = follow --color)
color      = "auto"       # "auto" | "always" | "never"
sort       = "name"       # "name" | "size" | "time" | "ext" | "none"
dirs_first = true
show_hidden = false
# depth    = 5            # uncomment to set a global depth limit
ignore     = [".git", "node_modules", "target", "__pycache__", ".idea", ".vscode"]
theme      = "default"

[themes.default.colors]
dir          = "blue"
file         = "white"
symlink      = "cyan"
exec         = "green"
image        = "magenta"
audio        = "magenta"
video        = "magenta"
archive      = "red"
config_file  = "yellow"
doc          = "bright_white"
git_modified  = "yellow"
git_untracked = "red"
git_staged   = "green"
git_ignored  = "bright_black"
meta         = "bright_black"
connector    = "bright_black"
count        = "bright_white"

# [themes.dark]
# colors.dir = "bright_blue"
# colors.exec = "bright_green"
"#
        .into()
    }

    /// Write the default config file to the platform config directory.
    pub fn generate_default() -> Result<PathBuf, String> {
        let dir = dirs::config_dir()
            .ok_or_else(|| "Cannot determine config directory".to_string())?
            .join("ssp");
        std::fs::create_dir_all(&dir)
            .map_err(|e| format!("Cannot create {}: {}", dir.display(), e))?;
        let path = dir.join("config.toml");
        std::fs::write(&path, Self::default_toml())
            .map_err(|e| format!("Cannot write {}: {}", path.display(), e))?;
        Ok(path)
    }
}
