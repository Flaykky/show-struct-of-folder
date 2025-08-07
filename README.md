## SSP – Show Structure of Project

A command-line tool for displaying directory structures with various filtering and formatting options.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Options](#options)
- [Examples](#examples)
- [Features](#features)
- [Configuration](#configuration)
- [Error Handling](#error-handling)

## Installation

To install SSP, you need to have Rust installed on your system. Clone the repository and build the project:

```bash
git clone <repository-url>
cd ssp
cargo build --release
```

The executable will be located at `target/release/ssp`.

## Usage

```bash
ssp [OPTIONS] [DIRECTORY]
```

If no directory is specified, SSP will display the structure of the current working directory.

## Options

| Flag | Long Form | Description | Example |
|------|-----------|-------------|---------|
| `-i` | `--ignore` | Ignore specified folder | `ssp -i node_modules` |
| `-of` | `--only-folders` | Show only directories | `ssp -of` |
| `-l` | `--lines` | Show line count for files | `ssp -l` |
| `-e` | `--extension` | Filter by file extension | `ssp -e rs` |
| `-d` | `--depth` | Limit display depth | `ssp -d 2` |
| `-h` | `--help` | Show help message | `ssp -h` |

### Detailed Option Descriptions

#### `--ignore` / `-i`
Ignores specified folders when displaying the directory structure. You can specify multiple folders by using the flag multiple times.

**Default ignored folders**: `.git`, `node_modules`, `__pycache__`

#### `--only-folders` / `-of`
Displays only directories, excluding all files from the output.

#### `--lines` / `-l`
Shows the number of lines in each file alongside the filename. This is particularly useful for code analysis.

#### `--extension` / `-e`
Filters files to show only those with the specified extension. For example, `-e rs` will show only Rust files.

#### `--depth` / `-d`
Limits the depth of directory traversal. For example, `-d 2` will show only up to 2 levels deep.

#### `--help` / `-h`
Displays the help message with usage instructions.

## Examples

### Basic Usage
```bash
# Display current directory structure
ssp

# Display specific directory structure
ssp /path/to/project
```

### Filtering Examples
```bash
# Show only folders
ssp -of

# Show only .rs files with line counts
ssp -l -e rs

# Ignore multiple folders
ssp -i node_modules -i target -i .git

# Limit depth to 2 levels
ssp -d 2

# Combine multiple options
ssp -of -d 3 -i build
```

### Sample Output

**Basic output:**
```
project/
├── src
│   ├── main.rs
│   └── utils.rs
├── tests
│   └── integration.rs
├── Cargo.toml
└── README.md
```

**With line counts:**
```
project/
├── src
│   ├── main.rs (150)
│   └── utils.rs (75)
├── tests
│   └── integration.rs (200)
├── Cargo.toml (25)
└── README.md (50)
```

**Only folders with limited depth:**
```
project/
├── src
│   ├── main
│   └── utils
└── docs
    ├── api
    └── guides
```

## Features

### Tree Structure Display
SSP displays directory structures using Unicode tree characters for clear visualization:
- `├──` for items with siblings below
- `└──` for the last item in a directory
- `│   ` for vertical continuation lines

### Automatic Sorting
Entries are automatically sorted with directories appearing first, followed by files, both in alphabetical order.

### Default Ignore Patterns
SSP automatically ignores common development folders:
- `.git` - Git metadata
- `node_modules` - Node.js dependencies
- `__pycache__` - Python cache files

### Cross-Platform Compatibility
Works on Windows, macOS, and Linux systems.

## Configuration

SSP currently doesn't require any configuration files. All settings are passed through command-line arguments.

## Error Handling

SSP provides clear error messages for common issues:

### Path Errors
- **Non-existent path**: `Error: Path 'nonexistent' does not exist`
- **Not a directory**: `Error: 'file.txt' is not a directory`

### Argument Errors
- **Missing argument**: `Error: --ignore flag requires an argument`
- **Invalid depth**: `Error: --depth flag requires a numeric value`
- **Unknown flag**: `Unknown flag: --invalid`

### File Access Errors
- **Permission denied**: Appropriate system error messages
- **Read errors**: `Failed to read directory` with details

## Performance Considerations

- SSP reads directory entries only when needed for display
- Large directories with many files may take longer to process
- Line counting for the `--lines` option requires reading entire files

## Limitations

- Does not follow symbolic links to avoid infinite loops
- Unicode tree characters may not display properly in all terminals
- Line counting is based on newline characters and may not match IDE counts exactly

## Contributing

To contribute to SSP:
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Version History

### v1.0.0
- Initial release
- Basic directory structure display
- Support for filtering options
- Line counting functionality
- Depth limiting
- Cross-platform support

## Troubleshooting

### Common Issues

**Tree characters not displaying correctly:**
- Ensure your terminal supports Unicode
- Check your locale settings

**Performance issues with large directories:**
- Use `--depth` to limit traversal
- Use `--ignore` to skip large dependency folders

**Permission errors:**
- Run with appropriate permissions
- Use `--ignore` to skip inaccessible directories

### Getting Help

For additional help, use the `--help` flag or check this documentation. For bug reports, please open an issue on the project repository.

## License

Distributed under the MIT License. See [LICENSE](LICENSE) file for details.
