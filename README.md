## SSP – Show Structure of Project

A small, configurable CLI utility in Rust to render a directory tree with customizable ASCII symbols via an external config file.

---

## 📌 Overview

`ssp` (short for **Show Structure of Project**) outputs a tree‑like visualization of a folder’s hierarchy. It:

* Skips common ignored directories (`.git`, `node_modules`, `__pycache__`, etc.).
* Lists directories first, then files, both sorted alphabetically.
* Supports multiple “display modes” (sets of ASCII connectors) via a simple TOML config.
* Lets you choose or switch modes on the fly with `--mode`.

---

## 🚀 Installation

1. **Clone the repo**

   ```bash
   git clone https://github.com/Flaykky/show-struct-of-folder
   cd show-struct-of-folder
   ```
2. **Build with Cargo**

   ```bash
   cargo build --release
   ```
3. **(Optional) Install system‑wide**

   ```bash
   sudo cp target/release/ssp /usr/local/bin/
   ```

---

## 🔧 Usage

```bash
# Show tree of current directory, using default mode
ssp

# Show tree of a specific folder
ssp ./src

# Specify a mode defined in config
ssp --mode=new
```

If no path is given, `ssp` defaults to the current working directory.

---

## 📁 Configuration

`ssp` supports a `ssp.toml` file in the current directory (or home directory) for mode definitions.

### Example `ssp.toml`

```toml
default_mode = "fancy"

[modes.old]
vertical = "│  "
tee      = "├──"
elbow    = "└──"
indent   = "    "

[modes.new]
vertical = "│  "
tee      = "╠══"
elbow    = "╚══"
indent   = "   "

[modes.fancy]
vertical = "┃   "
tee      = "┣━ "
elbow    = "┗━ "
indent   = "    "
```

* **`default_mode`**: (optional) name of the mode used when `--mode` is omitted.
* **`modes.<name>`**: each mode must define four fields:

  * `vertical`: the “│”‑style branch filler
  * `tee`: the middle‑branch connector (e.g. `├──`)
  * `elbow`: the last‑child connector (e.g. `└──`)
  * `indent`: the space inserted after branching

You can add as many modes as you like.

---

## ⚙️ Implementation Details

### `main()`

1. Parse `--mode=<name>` and optional path argument.
2. Load `ssp.toml` (or fall back to built‑in defaults).
3. Validate target path (must exist and be a directory).
4. Read entries, filter & sort, then print root and recurse.

### `filter_and_sort_entries()`

* Filters out directories starting with `.` plus `node_modules`, `__pycache__`, `.git`.
* Sorts so directories come before files, then alphabetically.

### `print_dir()` & `print_file()`

* Choose appropriate connector (`tee` vs. `elbow`) based on “is last child” and root status.
* Prepend `vertical` or `indent` to create proper nesting.
* Recursively descend into subdirectories.

---

## 🔍 Examples

### Default (built‑in “old”) mode

```text
my_project/
│  ├── src
│  │  ├── main.rs
│  │  └── lib.rs
│  └── Cargo.toml
```

### “new” mode (using `╠══` / `╚══`)

```bash
ssp --mode=new
```

```text
my_project/
╠══ src
║   ╠══ main.rs
║   ╚══ lib.rs
╚══ Cargo.toml
```

### “fancy” mode (custom in `ssp.toml`)

```bash
ssp --mode=fancy
```

```text
my_project/
┣━ src
┃   ┣━ main.rs
┃   ┗━ lib.rs
┗━ Cargo.toml
```

---

## ❗ Error Handling

* **Invalid path**:

  ```bash
  Error: '/foo/bar' does not exist
  ```
* **Not a directory**:

  ```bash
  Error: '/foo/file.txt' is not a directory
  ```
* **Unknown mode**:

  ```bash
  panic!("Mode 'xyz' not found in config")
  ```

---

## 📦 Dependencies

* **serde** + **serde\_derive** for deserializing TOML/JSON
* **toml** crate (or swap out for JSON)
* Standard Rust libraries: `std::env`, `std::fs`, `std::path`

---

## ✅ Summary

`ssp` is a flexible, mode‑driven tree viewer that you can tailor by editing a simple `ssp.toml`. Perfect for quickly inspecting nested folder structures with your preferred ASCII style.


## 🛠 Future Improvements

* Add CLI options for showing hidden files.
* Support for file type icons (UTF-8 based).
* Output to file (e.g., Markdown tree structure).
* Recode on other operating systems

## Installing the ssp into to **system path in Linux**
```bash
git clone https://github.com/Flaykky/show-struct-of-folder
cd show-struct-of-folder
rustc ssp.rs -o ssp
sudo mv ssp /usr/local/bin/ssp
```

## Requirments

* Rust compiler (rustc)
* Unix-like operating system


## License

Distributed under the MIT License. See [LICENSE](LICENSE) file for details.
