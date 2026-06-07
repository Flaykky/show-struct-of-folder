//! Static icon + color-class maps for file types.
//! Icons require a Nerd Font to render correctly.

/// Returns a Nerd Font icon for the given file extension (lowercase).
pub fn icon_for_ext(ext: &str) -> &'static str {
    match ext {
        // Rust
        "rs" => "󱘗",
        // Python
        "py" | "pyc" | "pyo" | "pyw" => "",
        // JavaScript / TypeScript
        "js" | "mjs" | "cjs" => "󰌞",
        "ts" | "mts" | "cts" => "󰛦",
        "jsx" => "",
        "tsx" => "",
        // Web
        "html" | "htm" => "",
        "css" => "",
        "scss" | "sass" => "",
        "vue" => "",
        "svelte" => "",
        // Config / data
        "json" => "",
        "jsonc" => "",
        "toml" => "",
        "yaml" | "yml" => "",
        "xml" => "󰗀",
        "csv" => "",
        "env" => "",
        // Docs
        "md" | "mdx" => "",
        "txt" => "󰈙",
        "pdf" => "",
        "rst" => "",
        // Shell
        "sh" | "bash" | "zsh" | "fish" | "ksh" => "",
        "ps1" | "psm1" | "psd1" => "",
        "bat" | "cmd" => "",
        // C family
        "c" => "",
        "h" => "",
        "cpp" | "cc" | "cxx" => "",
        "hpp" | "hxx" | "hh" => "",
        // Systems languages
        "go" => "",
        "zig" => "",
        "v" => "",
        // JVM
        "java" => "",
        "kt" | "kts" => "",
        "scala" => "",
        "groovy" => "",
        "class" | "jar" => "",
        // .NET
        "cs" => "󰌛",
        "fs" | "fsx" => "",
        "vb" => "",
        // Ruby / PHP / Perl / Lua / Erlang / Elixir / Haskell
        "rb" | "rake" | "gemspec" => "",
        "php" => "",
        "pl" | "pm" => "",
        "lua" => "",
        "ex" | "exs" => "",
        "erl" | "hrl" => "",
        "hs" | "lhs" => "",
        "clj" | "cljs" | "cljc" => "",
        // Swift / Objective-C
        "swift" => "",
        "m" | "mm" => "",
        // Other
        "r" | "rmd" => "󰟔",
        "dart" => "",
        "nim" => "",
        "cr" => "",
        // Images
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "ico" | "bmp" | "tiff" | "tif" | "svg" => "",
        "avif" | "heif" | "heic" => "",
        // Video / audio
        "mp4" | "mkv" | "webm" | "avi" | "mov" | "flv" | "wmv" => "",
        "mp3" | "ogg" | "flac" | "wav" | "aac" | "m4a" => "",
        // Archives
        "zip" | "tar" | "gz" | "bz2" | "xz" | "zst" | "7z" | "rar" => "",
        "deb" | "rpm" => "",
        // Binaries / executables
        "exe" | "dll" | "so" | "dylib" | "a" | "lib" => "",
        "wasm" => "",
        // Containers / infra
        "dockerfile" => "",
        "tf" | "tfvars" => "󱁢",
        "nix" => "",
        // Fonts
        "ttf" | "otf" | "woff" | "woff2" => "",
        // Databases
        "sql" => "",
        "db" | "sqlite" | "sqlite3" => "",
        // Locks
        "lock" => "",
        _ => "󰈔",
    }
}

/// Returns a Nerd Font icon for well-known file names (case-insensitive check done by caller).
pub fn icon_for_name(name: &str) -> Option<&'static str> {
    match name {
        ".gitignore" | ".gitattributes" | ".gitmodules" => Some(""),
        ".editorconfig" => Some(""),
        ".env" | ".env.local" | ".env.example" => Some(""),
        "dockerfile" | "containerfile" => Some(""),
        "docker-compose.yml" | "docker-compose.yaml" => Some(""),
        "makefile" | "gnumakefile" | "bsdmakefile" => Some(""),
        "justfile" => Some(""),
        "cargo.toml" | "cargo.lock" => Some("󱘗"),
        "package.json" | "package-lock.json" => Some(""),
        "yarn.lock" => Some(""),
        "pnpm-lock.yaml" => Some(""),
        "go.mod" | "go.sum" => Some(""),
        "pyproject.toml" | "setup.py" | "setup.cfg" | "requirements.txt" => Some(""),
        "readme" | "readme.md" | "readme.txt" | "readme.rst" => Some(""),
        "license" | "licence" | "license.txt" | "license.md" => Some(""),
        "changelog" | "changelog.md" | "changes" | "changes.md" => Some(""),
        ".travis.yml" => Some(""),
        ".github" => Some(""),
        "node_modules" => Some(""),
        "target" => Some("󱘗"),
        "__pycache__" => Some(""),
        "venv" | ".venv" | "env" => Some(""),
        ".git" => Some(""),
        "src" => Some(""),
        "tests" | "test" | "spec" => Some(""),
        "docs" | "doc" => Some(""),
        "dist" | "build" | "out" | "output" => Some(""),
        "scripts" | "script" => Some(""),
        "assets" | "static" | "public" => Some(""),
        _ => None,
    }
}

/// Icon for a directory (fallback).
pub const DIR_ICON: &str = "";
/// Icon for a symlink.
pub const SYMLINK_ICON: &str = "";
