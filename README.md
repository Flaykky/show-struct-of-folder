# SSP – Show Structure of Project

<div align="center">

![Version](https://img.shields.io/badge/version-0.1.1-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Rust](https://img.shields.io/badge/rust-1.70+-orange)

A powerful command-line tool for visualizing directory structures, analyzing code, and exporting project insights.

</div>

## 📑 Table of Contents

- [Features](#features)
- [Installation](#installation)
  - [From Source](#from-source)
  - [Quick Install (Linux/macOS)](#quick-install-linuxmacos)
  - [Quick Install (Windows)](#quick-install-windows)
- [Usage](#usage)
- [Options](#options)
- [Examples](#examples)
- [Configuration](#configuration)
- [Building](#building)
- [Uninstallation](#uninstallation)
- [Contributing](#contributing)
- [License](#license)

## ✨ Features

- 🌳 **Beautiful tree structure** visualization with Unicode characters
- 📊 **Code analysis** with detailed statistics
- 💾 **Export to file** for documentation
- 📝 **Code extraction** from all project files
- 🔍 **Smart filtering** by extension, depth, and folders
- 📈 **Line counting** for files
- 🚀 **Fast and lightweight** written in Rust
- 🌐 **Cross-platform** support (Linux, macOS, Windows)

## 🚀 Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### From Source

```bash
# Clone the repository
git clone https://github.com/Flaykky/show-struct-of-folder
cd show-struct-of-folder

# Build the project
cargo build --release

# The binary will be at: target/release/ssp
```

### Quick Install (Linux/macOS)

Run the installation script:

```bash
chmod +x install.sh
./install.sh
```

This will:
- Build the project in release mode
- Copy the binary to `~/.local/bin/`
- Add to PATH if needed
- Make it available system-wide

### Quick Install (Windows)

Run the installation script in PowerShell (as Administrator):

```powershell
.\install.ps1
```

This will:
- Build the project in release mode
- Copy the binary to `C:\Program Files\ssp\`
- Add to system PATH
- Make it available system-wide

**Alternative (without admin rights):**

```powershell
.\install.ps1 -UserInstall
```

This installs to `%USERPROFILE%\.local\bin\` instead.

## 📖 Usage

```bash
ssp [OPTIONS] [DIRECTORY]
```

If no directory is specified, SSP analyzes the current working directory.

## ⚙️ Options

| Short | Long | Description | Example |
|-------|------|-------------|---------|
| `-i` | `--ignore` | Ignore specified folder | `ssp -i node_modules` |
| `-of` | `--only-folders` | Show only directories | `ssp -of` |
| `-l` | `--lines` | Show line count for files | `ssp -l` |
| `-e` | `--extension` | Filter by file extension | `ssp -e rs` |
| `-d` | `--depth` | Limit display depth | `ssp -d 2` |
| `-o` | `--output` | Save output to file | `ssp -o struct.txt` |
| `-sc` | `--show-code` | Show code from all files | `ssp -sc` |
| `-a` | `--analyze` | Analyze code statistics | `ssp -a` |
| `-h` | `--help` | Show help message | `ssp -h` |

### 🔍 Detailed Descriptions

#### `--ignore` / `-i`
Exclude specific folders from the output. Can be used multiple times.

**Default ignored:** `.git`, `node_modules`, `__pycache__`, `target`, `.idea`, `.vscode`

```bash
ssp -i build -i dist -i tmp
```

#### `--only-folders` / `-of`
Display only directories, hiding all files.

```bash
ssp -of
```

#### `--lines` / `-l`
Display the number of lines next to each file.

```bash
ssp -l
# Output: main.rs (150)
```

#### `--extension` / `-e`
Show only files with a specific extension.

```bash
ssp -e py    # Python files only
ssp -e js    # JavaScript files only
```

#### `--depth` / `-d`
Limit how deep the tree traverses.

```bash
ssp -d 2    # Show only 2 levels
```

#### `--output` / `-o`
Save the entire output to a file instead of printing to console.

```bash
ssp -o project_structure.txt
ssp -sc -a -o full_report.md
```

#### `--show-code` / `-sc`
Extract and display the code content from all files in the project.

```bash
ssp -sc
```

**Output format:**
```
project/
├── src
│   ├── main.rs
│   └── utils.rs
└── Cargo.toml

=== CODE CONTENT ===

1. src/main.rs:

fn main() {
    println!("Hello, world!");
}

--------------------------------------------------------------------------------

2. src/utils.rs:

pub fn helper() -> i32 {
    42
}
```

#### `--analyze` / `-a`
Perform detailed code analysis with statistics.

```bash
ssp -a
```

**Analysis includes:**
- Total files and lines
- Blank and comment lines
- Files grouped by extension
- Lines per extension
- Function and class counts
- Type declarations (int, float, string, bool)
- Code density percentage

## 📚 Examples

### Basic Usage

```bash
# Current directory structure
ssp

# Specific directory
ssp /path/to/project

# With line counts
ssp -l
```

### Filtering

```bash
# Only folders
ssp -of

# Only Rust files
ssp -e rs

# Python files with line counts
ssp -e py -l

# First 3 levels only
ssp -d 3

# Ignore multiple folders
ssp -i build -i dist -i __pycache__
```

### Code Analysis

```bash
# Full analysis
ssp -a

# Analysis with code content
ssp -sc -a

# Analyze specific file type
ssp -e rs -a

# Save analysis to file
ssp -a -o analysis.txt
```

### Export & Documentation

```bash
# Save structure to file
ssp -o structure.txt

# Full project documentation
ssp -l -sc -a -o full_docs.md

# Quick reference with code
ssp -d 2 -sc -o quick_ref.txt
```

### Combined Examples

```bash
# Python project analysis
ssp -e py -l -a -o python_analysis.txt

# Frontend structure (ignore build artifacts)
ssp -i node_modules -i dist -i build -of

# Rust project with code and stats
ssp -e rs -sc -a -o rust_project.md

# Quick overview (2 levels, folders only)
ssp -d 2 -of
```

## 📊 Sample Outputs

### Basic Structure
```
my-project/
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── utils
│       ├── parser.rs
│       └── formatter.rs
├── tests
│   └── integration.rs
├── Cargo.toml
└── README.md
```

### With Line Counts
```
my-project/
├── src
│   ├── main.rs (150)
│   ├── lib.rs (200)
│   └── utils
│       ├── parser.rs (320)
│       └── formatter.rs (180)
├── tests
│   └── integration.rs (450)
├── Cargo.toml (25)
└── README.md (100)
```

### Analysis Output
```
=== CODE ANALYSIS ===

Total Files: 12
Total Lines: 2,847
Blank Lines: 342
Comment Lines: 518
Code Lines: 1,987

Files by Extension:
  .rs: 8 files
  .toml: 2 files
  .md: 2 files

Lines by Extension:
  .rs: 2,450 lines
  .toml: 147 lines
  .md: 250 lines

Code Elements (approximate):
  Functions: 67
  Classes/Structs: 15
  Int declarations: 23
  Float declarations: 8
  String declarations: 89
  Bool declarations: 34

Code Density: 69.8%
```

## 🔧 Configuration

SSP doesn't require configuration files. All settings are passed via command-line arguments.

### Default Ignored Folders

- `.git` - Git repository data
- `node_modules` - Node.js dependencies
- `__pycache__` - Python cache
- `target` - Rust build artifacts
- `.idea` - IntelliJ IDEA
- `.vscode` - Visual Studio Code

You can override these with `-i` flag.

## 🛠️ Building

### Development Build

```bash
cargo build
./target/debug/ssp
```

### Release Build

```bash
cargo build --release
./target/release/ssp
```

### Run Without Installing

```bash
cargo run -- [OPTIONS] [DIRECTORY]

# Examples:
cargo run -- -l
cargo run -- -a /path/to/project
```

### Run Tests

```bash
cargo test
```

## 📦 Creating Distribution Package

### Linux (DEB)

```bash
./scripts/create_deb.sh
# Creates ssp_2.0.0_amd64.deb
```

### Linux (RPM)

```bash
./scripts/create_rpm.sh
# Creates ssp-2.0.0.rpm
```

### Windows (Installer)

```powershell
.\scripts\create_installer.ps1
# Creates ssp-installer.exe
```

## 🗑️ Uninstallation

### Linux/macOS

```bash
./uninstall.sh
```

Or manually:
```bash
rm ~/.local/bin/ssp
```

### Windows

```powershell
.\uninstall.ps1
```

Or manually:
- Remove `C:\Program Files\ssp\` (or `%USERPROFILE%\.local\bin\`)
- Remove from PATH environment variable

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust naming conventions
- Add tests for new features
- Update documentation
- Keep code clean and commented

## 📝 Roadmap

- [ ] Package for apt/dnf/pacman
- [ ] Add ignore patterns from `.gitignore`
- [ ] Custom output formats (JSON, XML)
- [ ] Syntax highlighting in code output
- [ ] Git integration (show file status)
- [ ] More detailed code metrics
- [ ] Plugin system
- [ ] Configuration file support
- [ ] Interactive TUI mode

## 🐛 Troubleshooting

### Tree characters not displaying

**Issue:** Box drawing characters appear as `?` or weird symbols.

**Solution:**
- Ensure terminal supports UTF-8
- Set locale: `export LANG=en_US.UTF-8`
- Use a modern terminal (iTerm2, Windows Terminal, Alacritty)

### Permission denied errors

**Issue:** Cannot read certain directories.

**Solution:**
- Run with appropriate permissions
- Use `-i` to ignore problematic directories

### Slow performance on large projects

**Issue:** Takes too long to analyze.

**Solution:**
- Use `-d` to limit depth
- Use `-i` to ignore large folders (node_modules, target)
- Use `-e` to filter by extension


## 👏 Acknowledgments

- Inspired by the `tree` command
- Built with ❤️ using Rust
- Thanks to all contributors

## 📧 Contact

- **Author:** Flaykky
- **GitHub:** [@Flaykky](https://github.com/Flaykky)
- **Repository:** [show-struct-of-folder](https://github.com/Flaykky/show-struct-of-folder)

---

<div align="center">

**Made with Rust**

If you find this tool useful, please ⭐ star the repository!

</div>


## license 

Distributed under the MIT License. See [LICENSE](LICENSE) file for details.


