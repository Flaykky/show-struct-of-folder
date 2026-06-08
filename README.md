# SSP – Show Structure of Project

<div align="center">

![Version](https://img.shields.io/badge/version-2.0.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Rust](https://img.shields.io/badge/rust-1.70+-orange)

A modern, configurable alternative to `tree` — with colors, icons, git integration,
multiple output formats, and flexible filtering.

</div>

## ✨ Features

- 🌳 **Beautiful tree** with Unicode box-drawing (or `--ascii` fallback)
- 🎨 **ANSI colors + Nerd Font icons** — auto-detected, fully themeable
- 🌿 **Git integration** — respect `.gitignore` by default, show per-file status markers
- 🔍 **Flexible filtering** — hidden files, glob patterns, extension, depth, prune
- 🔀 **Sorting** — by name, size, modification time, or extension; reversible
- 📤 **Multiple output formats** — tree, JSON, Markdown, flat list
- 📊 **Code analysis** — line counts, blank/comment/code breakdown, function/struct counts
- 💾 **Export to file** — pipe any format to a file with `-o`
- ⚙️ **TOML config + themes** — persistent defaults, named color themes
- 🚀 **Fast** — built with Rust, uses ripgrep's `ignore` crate for traversal
- 🌐 **Cross-platform** — Linux, macOS, other unix-likes, Windows

## 🚀 Installation

### Prerequisites

- Rust 1.70+  (`rustup.rs`)

### From Source

```bash
git clone https://github.com/Flaykky/show-struct-of-folder
cd show-struct-of-folder
cargo build --release
# binary at: target/release/ssp
```

### Quick Install (Linux / macOS)

```bash
chmod +x install.sh
./install.sh          # installs to ~/.local/bin/ssp
```

### Quick Install (Windows — PowerShell as Admin)

```powershell
.\win_install.ps1             # installs to C:\Program Files\ssp\
.\win_install.ps1 -UserInstall  # installs to %USERPROFILE%\.local\bin\
```

### Via `cargo install` (any platform)

```bash
cargo install --git https://github.com/Flaykky/show-struct-of-folder
```

## 📖 Usage

```
ssp [OPTIONS] [DIRECTORY]
```

If no directory is given, the current working directory is used.

## ⚙️ Options

### Display

| Flag | Description |
|------|-------------|
| `--color <auto\|always\|never>` | ANSI color mode (default: `auto`) |
| `--icons <auto\|always\|never>` | Nerd Font icons (default: `auto`) |
| `--no-icons` | Disable icons |
| `--ascii` | ASCII connectors instead of Unicode (`|-- `, `` `-- ``) |
| `-f, --full-path` | Show path relative to root for each entry |

### Depth

| Flag | Description |
|------|-------------|
| `-d, --depth <N>` | Limit display depth (alias: `-L`) |

### Filtering

| Flag | Description |
|------|-------------|
| `-a, --all` | Show hidden files (starting with `.`) |
| `--no-gitignore` | Ignore `.gitignore` rules (respected by default) |
| `-i, --ignore <NAME>` | Ignore a folder by name (repeatable) |
| `-P, --pattern <GLOB>` | Include only files matching a glob (repeatable) |
| `-I, --ignore-glob <GLOB>` | Exclude entries matching a glob (repeatable) |
| `-e, --extension <EXT>` | Show only files with this extension |
| `-D, --dirs-only` | Show only directories |
| `--files-only` | Show only files |
| `--prune` | Hide empty directories |

**Default ignored:** `.git`, `node_modules`, `target`, `__pycache__`, `.idea`, `.vscode`
(override with `--ignore` or via config file)

### Sorting

| Flag | Description |
|------|-------------|
| `-s, --sort <name\|size\|time\|ext\|none>` | Sort key (default: `name`) |
| `-r, --reverse` | Reverse sort order |
| `--dirs-first` | List directories before files (default: on) |
| `--no-dirs-first` | Mix directories and files in sort order |

### Git

| Flag | Description |
|------|-------------|
| `--git` | Show git status markers per entry |

Status markers: `●` staged · `✚` modified · `?` untracked · `!` ignored

### Metadata

| Flag | Description |
|------|-------------|
| `-l, --lines` | Show line count next to each file |
| `--sizes` | Show file sizes |
| `--summary` | Print total directories, files, and size |

### Code Analysis

| Flag | Description |
|------|-------------|
| `-A, --analyze` | Analyze code: lines, comments, functions, types |
| `--show-code` | Print full file contents after the tree |

### Output

| Flag | Description |
|------|-------------|
| `-o, --output <FILE>` | Write output to a file |
| `--format <tree\|json\|markdown\|list>` | Output format (default: `tree`) |

### Config / Themes

| Flag | Description |
|------|-------------|
| `--config <FILE>` | Use a specific config file |
| `--theme <NAME>` | Select a named theme from the config |
| `--no-config` | Ignore the config file entirely |
| `--generate-config` | Write a default config to `~/.config/ssp/config.toml` |

## 🎨 Config File & Themes

The config file is auto-loaded from:

1. `--config <FILE>`
2. `$SSP_CONFIG` environment variable
3. `~/.config/ssp/config.toml` (Linux/macOS) / `%APPDATA%\ssp\config.toml` (Windows)

Generate the default config:

```bash
ssp --generate-config
```

Example `config.toml`:

```toml
[defaults]
icons      = true
color      = "auto"       # auto | always | never
sort       = "name"       # name | size | time | ext | none
dirs_first = true
show_hidden = false
ignore     = [".git", "node_modules", "target"]
theme      = "default"

[themes.default.colors]
dir      = "blue"
file     = "white"
symlink  = "cyan"
exec     = "green"
archive  = "red"
meta     = "bright_black"

# [themes.dark.colors]
# dir = "bright_blue"
```

## 📚 Examples

```bash
# Basic tree
ssp
ssp /path/to/project

# Show hidden files, 3 levels deep
ssp -a -d 3

# Only Rust files with line counts
ssp -e rs -l

# Sort by size, largest first
ssp --sizes -s size -r

# Glob: only config files
ssp -P '*.toml' -P '*.yaml'

# Exclude build artifacts
ssp -I '*.lock' -I '*.log'

# Git status markers
ssp --git

# Full analysis + code content → file
ssp -A --show-code -o report.md

# JSON output (pipe to jq)
ssp --format json | jq '.children[].name'

# Markdown outline
ssp --format markdown -d 2

# Flat path list (for scripting)
ssp --format list | grep '\.rs$'

# ASCII mode (safe for all terminals)
ssp --ascii

# Summary line
ssp --summary

# Specific theme
ssp --theme dark
```

## 📊 Sample Output

### Tree (default)

```
my-project/
├── src/
│   ├── main.rs (250 lines)
│   └── lib.rs (180 lines)
├── tests/
│   └── integration.rs (90 lines)
├── Cargo.toml
└── README.md

1 directories, 5 files, 28.3K
```

### Code Analysis (`-A`)

```
=== CODE ANALYSIS ===

Total Files:   11
Total Lines:   1921
Blank Lines:   312
Comment Lines: 183
Code Lines:    1426
Code Density:  74.2%

Files by Extension:
  .rs         11 files

Lines by Extension:
  .rs         1921 lines

Code Elements (approximate):
  Functions:        61
  Classes/Structs:  14
  Int declarations: 33
  Float decls:      4
  String decls:     58
  Bool decls:       54
```

### JSON (`--format json`)

```json
{
  "name": "my-project",
  "type": "directory",
  "children": [
    { "name": "src", "type": "directory", "children": [...] },
    { "name": "Cargo.toml", "type": "file" }
  ]
}
```

## 🔧 Default Ignored Folders

`.git` · `node_modules` · `target` · `__pycache__` · `.idea` · `.vscode`

Override via `-i <name>` (add extra) or set `ignore = [...]` in `config.toml` (replace).

## 🛠️ Building

```bash
cargo build              # debug
cargo build --release    # release (target/release/ssp)
cargo test               # run tests
cargo clippy             # lint
```

## 📝 Roadmap

- [ ] Per-file-type coloring (exec, image, archive)
- [ ] LS_COLORS environment variable support
- [ ] Native git2 integration (no `git` binary needed)
- [ ] Interactive TUI / fuzzy navigation
- [ ] Package for apt/dnf/pacman/homebrew
- [ ] Syntax highlighting in `--show-code` output
- [ ] Plugin/script hooks

## 🐛 Troubleshooting

**Icons show as `?` boxes** — install a [Nerd Font](https://www.nerdfonts.com) and configure
your terminal to use it, or pass `--no-icons`.

**No colors** — ensure your terminal supports ANSI. Pass `--color always` to force them.
On Windows, run in Windows Terminal or PowerShell 7+.

**`.gitignore` not respected** — by default gitignore IS respected. Pass `--no-gitignore`
to disable it.

**Slow on very large repos** — use `-d` to limit depth and `-i` to skip heavy directories.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes
4. Open a Pull Request

Please follow Rust naming conventions and run `cargo clippy` before submitting.

## 📧 Contact

- **Author:** Flaykky
- **GitHub:** [@Flaykky](https://github.com/Flaykky)
- **Repository:** [show-struct-of-folder](https://github.com/Flaykky/show-struct-of-folder)

## 📝 License

MIT — see [LICENSE](LICENSE).
