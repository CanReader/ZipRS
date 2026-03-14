<p align="center">
  <img src="icons/Logo.png" alt="ZipRS Logo" width="140" />
</p>

<h1 align="center">ZipRS</h1>

<p align="center">
  <strong>A fast, modern archive manager built with Rust</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.75+-orange?logo=rust" alt="Rust" />
  <img src="https://img.shields.io/badge/tauri-v2-blue?logo=tauri" alt="Tauri v2" />
  <img src="https://img.shields.io/badge/svelte-5-ff3e00?logo=svelte" alt="Svelte 5" />
  <img src="https://img.shields.io/badge/tailwindcss-4-38bdf8?logo=tailwindcss" alt="TailwindCSS" />
  <img src="https://img.shields.io/badge/license-MIT-green" alt="MIT License" />
</p>

---

<p align="center">
  <img src="Screenshot.jpg" alt="ZipRS Screenshot" width="800" />
</p>

## About

ZipRS is a professional desktop archive manager inspired by WinRAR, built entirely in Rust for maximum performance. It features both a modern dark GUI powered by Tauri v2, Svelte 5, and TailwindCSS 4, and a powerful command-line interface for scripting and terminal workflows.

## Features

### GUI
- **Multi-format support** — ZIP, TAR, TAR.GZ, TAR.BZ2, TAR.ZST
- **Full archive operations** — Open, browse, extract, add, delete, create, test
- **ZIP with Zstd compression** — Level 19 for maximum compression ratio
- **Folder navigation** — Breadcrumb address bar with back/forward/up history
- **File table** — Sortable columns, multi-select (Ctrl+Click, Shift+Click), colored file-type icons
- **Drag & drop** — Drop archives onto the window to open them
- **Drag out** — Extract files by dragging them out to your file manager
- **Context menu** — Right-click for quick actions on selected entries
- **Keyboard shortcuts** — Ctrl+O, Ctrl+N, Ctrl+E, Ctrl+T, Delete, F5, and more
- **Progress tracking** — Real-time progress bar for all archive operations
- **Properties panel** — Detailed file info including CRC-32, compression ratio, method
- **Integrity testing** — Verify archive contents with one click

### CLI
- **`ziprs list`** — List archive contents with optional long format and filtering
- **`ziprs extract`** — Extract all or specific files to a directory
- **`ziprs create`** — Create archives in any supported format
- **`ziprs add`** — Add files to an existing ZIP archive
- **`ziprs delete`** — Remove entries from a ZIP archive
- **`ziprs test`** — Verify archive integrity
- **`ziprs info`** — Detailed archive information and statistics
- **`ziprs tree`** — Display contents as a visual tree with sizes
- **`ziprs search`** — Find entries matching a pattern with highlighting
- **`ziprs cat`** — Print file contents from archive to stdout
- **`ziprs convert`** — Convert between formats (e.g. ZIP to TAR.GZ)
- **`ziprs diff`** — Compare two archives and show differences
- **`ziprs stats`** — Size distribution, compression stats, top largest files
- **`ziprs types`** — File type breakdown with visual bar chart

## Supported Formats

| Format | Browse | Extract | Add/Delete | Create |
|--------|--------|---------|------------|--------|
| ZIP    | Yes    | Yes     | Yes        | Yes    |
| TAR    | Yes    | Yes     | No         | Yes    |
| TAR.GZ | Yes    | Yes     | No         | Yes    |
| TAR.BZ2| Yes    | Yes     | No         | Yes    |
| TAR.ZST| Yes    | Yes     | No         | Yes    |

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/CanReader/ZipRS.git
cd ZipRS

# Install frontend dependencies
npm install

# Build for production
npm run build

# The binary is at target/release/ziprs
```

### Install Locally

```bash
# Symlink to your PATH
ln -sf $(pwd)/target/release/ziprs ~/.local/bin/ziprs
```

### System Dependencies (Linux)

```
webkit2gtk-4.1 libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

### Optional

- [ripdrag](https://github.com/nik012003/ripdrag) — enables drag-out file extraction from the GUI

## Usage

### GUI Mode

```bash
# Launch the graphical interface
ziprs
```

### CLI Mode

```bash
# List archive contents
ziprs list archive.zip
ziprs list -l archive.tar.gz            # detailed view
ziprs list archive.zip -f "*.rs"         # filter by pattern

# Extract
ziprs extract archive.zip                # extract to current dir
ziprs extract archive.zip -o ./output    # extract to specific dir
ziprs extract archive.zip --files src/main.rs src/lib.rs

# Create
ziprs create backup.zip file1.txt dir/   # create ZIP
ziprs create backup.tar.gz src/          # create TAR.GZ

# Modify (ZIP only)
ziprs add archive.zip newfile.txt
ziprs delete archive.zip old/unwanted.txt

# Inspect
ziprs info archive.zip                   # format, size, compression ratio
ziprs tree archive.zip -s                # visual tree with sizes
ziprs tree archive.zip -d 2             # tree limited to depth 2
ziprs stats archive.zip -n 5             # stats with top 5 largest files
ziprs types archive.zip                  # file type breakdown

# Search & View
ziprs search archive.zip "main"          # find entries matching pattern
ziprs search archive.zip "config" -l     # search with sizes and dates
ziprs cat archive.zip src/main.rs        # print file to stdout

# Convert & Compare
ziprs convert data.zip data.tar.gz       # convert between formats
ziprs diff v1.zip v2.zip                 # compare two archives

# Test
ziprs test archive.zip                   # verify integrity
```

All commands support `-v` for verbose progress output. Use `ziprs help <command>` for detailed usage.

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+O` | Open archive |
| `Ctrl+N` | Create new archive |
| `Ctrl+E` | Extract all |
| `Ctrl+H` | Extract here |
| `Ctrl+T` | Test archive integrity |
| `Ctrl+A` | Select all |
| `Delete` | Delete selected entries |
| `Enter` | Open file / Navigate into folder |
| `Backspace` | Navigate up |
| `Alt+Left/Right` | Navigate back/forward |
| `Alt+Enter` | Properties |
| `F5` | Refresh |

## Architecture

```
ZipRS/
├── src-tauri/                # Rust backend (Tauri v2)
│   └── src/
│       ├── main.rs           # Entry point (CLI/GUI dispatch)
│       ├── lib.rs            # Tauri builder & plugin setup
│       ├── cli.rs            # CLI commands (clap)
│       ├── commands.rs       # GUI IPC command handlers
│       ├── progress.rs       # Progress trait (GUI events / CLI stderr)
│       └── archive/
│           ├── mod.rs        # ArchiveBackend trait & format detection
│           ├── entry.rs      # ArchiveEntry data model
│           ├── zip_backend.rs    # ZIP (Zstd level 19)
│           └── tar_backend.rs    # TAR / GZ / BZ2 / ZST
├── src/                      # Svelte 5 frontend
│   ├── App.svelte            # Main app component
│   ├── app.css               # Dark theme & CSS variables
│   ├── main.ts               # Svelte mount
│   └── lib/
│       ├── store.svelte.ts   # Reactive state (Svelte 5 runes)
│       ├── types.ts          # TypeScript interfaces
│       ├── utils.ts          # Formatting & icon color helpers
│       └── components/
│           ├── WelcomeScreen.svelte
│           ├── Toolbar.svelte
│           ├── AddressBar.svelte
│           ├── FileTable.svelte
│           ├── StatusBar.svelte
│           ├── ContextMenu.svelte
│           ├── ProgressDialog.svelte
│           └── Dialogs.svelte
└── static/                   # Static assets
```

## Development

```bash
npm install          # install dependencies
npm run dev          # development mode with hot reload
npm run build        # production build
npm run check        # type-check Svelte + Rust
npm run lint         # run Clippy
npm run clean        # remove build artifacts
```

## Tech Stack

- **Backend** — Rust, Tauri v2, Tokio, clap, zip, tar, flate2, bzip2, zstd, colored
- **Frontend** — Svelte 5, TailwindCSS 4, Lucide icons, TypeScript
- **Build** — Vite 7, Tauri CLI

## License

MIT
