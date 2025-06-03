# SSP - Show Structure of Project

A simple CLI utility written in Rust to display the directory structure of a project in a tree-like format.

## 📌 Overview

`ssp` (short for **Show Structure of Project**) displays a visual representation of a folder's contents using ASCII tree symbols, similar to the Unix `tree` command. It skips common ignored folders like `.git` and `node_modules`, and visually distinguishes directories from files.

Example output when run inside a project folder named `CVPN`:

```text
CVPN/
│── include/
│   ├── tunnel.h
│   ├── transport.h
│   ├── crypto.h
│   ├── config.h
│   └── log.h
│
│── src/
│   ├── main.c
│   ├── tunnel.c
│   ├── transport.c
│   ├── crypto.c
│   ├── config.c
│   └── log.c
│
│── config/
│   ├── client.conf
│   └── server.conf
│
├── CMakeLists.txt
├── README.md
├── LICENSE
└── .gitignore
```

## 🚀 Usage

```bash
ssp                # Show structure of current directory
ssp ./include      # Show structure of ./include directory
```

## 📁 Features

* Display root directory name.
* Tree-like indentation for files and folders.
* Directories are listed before files.
* Hidden/system folders like `.git`, `node_modules`, and `__pycache__` are excluded.

## 🔧 Implementation Details

### Entry Point: `main()`

* Parses command line arguments.
* Defaults to current directory if no path is provided.
* Validates the input path.
* Lists entries, filters, and sorts them.
* Calls appropriate recursive functions for display.

### `filter_and_sort_entries()`

* Removes hidden/system folders.
* Sorts directories before files.
* Performs alphabetical sort within each group.

### `print_dir_structure()`

* Recursively prints folder contents.
* Applies appropriate indentation and connector symbols (`├──`, `└──`, etc.).
* Calls itself for subdirectories.

### `print_file_structure()`

* Prints individual file entries with tree connector.

## 📦 Dependencies

* Uses standard Rust libraries:

  * `std::env`
  * `std::fs`
  * `std::path`

## 📄 Example

Run from inside a project root:

```bash
ssp
```

Run for a specific directory:

```bash
ssp src
```

## ❗ Errors

* Invalid path or missing directory:

```bash
Error: Path './notfound' does not exist
Error: './file.txt' is not a directory
```

## ✅ Summary

`ssp` is a handy tool for developers who want a clean, structured overview of their project folders from the command line. Written in Rust, it is fast and efficient, with a clear output style.

## 🛠 Future Improvements

* Add CLI options for showing hidden files.
* Support for file type icons (UTF-8 based).
* Output to file (e.g., Markdown tree structure).
* Recode on other operating systems

## Installing sso to **system path in Linux**
```bash
git clone https://github.com/Flaykky/show-struct-of-folder
cd show-struct-of-folder
rustc ssp.rs -o ssp
sudo mv ssp /usr/local/bin/ssp
```


## License

Distributed under the MIT License. See [LICENSE](LICENSE) file for details.
