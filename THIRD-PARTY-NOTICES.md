# Third-Party Notices

ZipRS incorporates third-party libraries and components. The following is a
summary of the licenses for the major dependencies.

---

## Rust Dependencies

### tauri (MIT / Apache-2.0)
Cross-platform application toolkit.
https://github.com/tauri-apps/tauri

### zip (MIT)
ZIP archive reading and writing.
https://github.com/zip-rs/zip2

### tar (MIT / Apache-2.0)
TAR archive reading and writing.
https://github.com/alexcrichton/tar-rs

### flate2 (MIT / Apache-2.0)
DEFLATE, gzip, and zlib compression.
https://github.com/rust-lang/flate2-rs

### bzip2 (MIT / Apache-2.0)
BZip2 compression bindings.
https://github.com/alexcrichton/bzip2-rs

### zstd (MIT)
Zstandard compression bindings.
https://github.com/gyscos/zstd-rs

### unrar (MIT)
RAR archive extraction. Links against the UnRAR library which is
Copyright (c) Alexander L. Roshal. The UnRAR source code may be used in any
software to handle RAR archives without limitations free of charge, but cannot
be used to develop a RAR-compatible archiver (i.e., creating RAR archives).
https://github.com/muja/unrar.rs

### clap (MIT / Apache-2.0)
Command-line argument parser.
https://github.com/clap-rs/clap

### serde (MIT / Apache-2.0)
Serialization/deserialization framework.
https://github.com/serde-rs/serde

### tokio (MIT)
Asynchronous runtime.
https://github.com/tokio-rs/tokio

### chrono (MIT / Apache-2.0)
Date and time library.
https://github.com/chronotope/chrono

---

## JavaScript Dependencies

### Svelte (MIT)
Frontend UI framework.
https://github.com/sveltejs/svelte

### Tailwind CSS (MIT)
Utility-first CSS framework.
https://github.com/tailwindlabs/tailwindcss

### Lucide (ISC)
Icon library.
https://github.com/lucide-icons/lucide

---

## Runtime Components

### Microsoft Edge WebView2 (Redistributable)
Used for rendering the application UI on Windows.
https://developer.microsoft.com/en-us/microsoft-edge/webview2/

---

For complete license texts of all dependencies, run:
```
cargo license --manifest-path src-tauri/Cargo.toml
```
