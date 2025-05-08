# Duplicate Cleaner

A fast, cross-platform, GUI-based Rust application to find and delete duplicate files (by content hash), prioritize storage savings, and eventually support cloud integration (Google Drive, iCloud, etc).

> Built with Rust + EGUI + rayon + sha2. Focused on clean architecture, ease of use, and high performance.

---

## ✨ Features

- Identify duplicate files by SHA-256 hash
- Recursively scan any folder
- Filter by minimum file size (default: 1MB)
- Group duplicates and optionally delete redundant copies
- Cross-platform (Windows/macOS/Linux)
- Extensible architecture (cloud provider plugins coming soon)

---

## 🧰 Prerequisites

- **Rust**: Install using [rustup.rs](https://rustup.rs)
- **Git** (if cloning)
- **VS Code** + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) recommended

---

## 🚀 Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/yourusername/duplicatefilefinder.git
cd duplicatefilefinder
```

### 2. Install dependencies

```bash
cargo build
```

> Required crates are defined in `Cargo.toml`:

- `eframe` / `egui` – GUI framework
- `walkdir` – Recursive folder scanner
- `rayon` – Parallel hashing
- `sha2` – Hashing for file content

### 3. Run the app

```bash
cargo run
```

---

## 🗂️ Project Structure

```plaintext
src/
├── main.rs                   # Entry point with eframe setup
├── core/                    # Core logic: scanning, hashing, data models
│   ├── mod.rs
│   ├── scanner.rs           # Recursive filesystem traversal
│   ├── hashing.rs           # Parallel SHA-256 hash computation
│   └── models.rs            # Shared structs (e.g. FileEntry, FileSource)
├── gui/                     # GUI application (EGUI)
│   ├── mod.rs
│   └── app.rs               # DuplicateApp struct, UI layout and behavior
├── providers/               # Cloud provider stubs (Google Drive, iCloud)
│   ├── mod.rs
│   ├── google_drive.rs
│   └── icloud.rs
```

---

## 🧠 Contributing

We welcome PRs and collaboration!

### Contributor Setup

1. Ensure you can run the app (`cargo run`)
2. Use `rust-analyzer` or `cargo check` to validate changes
3. Use `cargo fmt` before committing
4. New modules? Register them in their `mod.rs` file

### Current Needs

| Feature                  | Status         |
| ------------------------ | -------------- |
| Local duplicate scanning | ✅ Done        |
| File deletion UI         | ⏳ In progress |
| Folder size analysis     | 🧩 Planned     |
| Google Drive integration | 🧩 Stubbed     |
| iCloud support           | 🧩 Stubbed     |
| Config file / JSON save  | 🧩 Planned     |

---

## 🔒 Security & Permissions

- No files are deleted automatically. All deletions require explicit user clicks.
- No internet connections are made unless a user opts into cloud integration.
- File paths are not uploaded or logged externally.

---

## 📜 License

MIT License

---

## 📬 Contact

For questions or to join the project:

- Open an issue or PR on GitHub
- Or reach out to [jknight](https://github.com/youruserjknight137)
