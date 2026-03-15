<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import {
    Folder, File, FileText, FileImage, FileCode, FileArchive, FileVideo, FileAudio,
    FileSpreadsheet, FileJson, FileType, FileKey, FileLock, FileCog, FileTerminal,
    FileHeart, Database, Globe, Presentation, Clapperboard, Music, Image,
    Box, Binary, Shield, Hash, Braces, Terminal, Cpu, HardDrive, Gamepad2, Pen
  } from "lucide-svelte";
  import { store } from "$lib/store.svelte";
  import { formatSize, formatRatio, fileTypeDisplay, getFileIconColor } from "$lib/utils";
  import type { SortColumn } from "$lib/types";

  interface Props {
    onAction: (action: string) => void;
    onContextMenu: (e: MouseEvent, index: number) => void;
  }

  let { onAction, onContextMenu }: Props = $props();
  let isDragging = $state(false);

  function handleRowClick(e: MouseEvent, index: number) {
    if (e.ctrlKey || e.metaKey) {
      const pos = store.selectedIndices.indexOf(index);
      if (pos >= 0) {
        store.selectedIndices = store.selectedIndices.filter((i) => i !== index);
      } else {
        store.selectedIndices = [...store.selectedIndices, index];
      }
      store.lastClickedIndex = index;
    } else if (e.shiftKey && store.lastClickedIndex !== null) {
      const start = Math.min(store.lastClickedIndex, index);
      const end = Math.max(store.lastClickedIndex, index);
      store.selectedIndices = Array.from({ length: end - start + 1 }, (_, i) => start + i);
    } else {
      store.selectedIndices = [index];
      store.lastClickedIndex = index;
    }
  }

  function handleRowDblClick(index: number) {
    const entry = store.visibleEntries[index];
    if (entry.is_directory) {
      store.navigateTo(entry.path);
    } else {
      store.selectedIndices = [index];
      onAction("open-file");
    }
  }

  function handleContextMenu(e: MouseEvent, index: number) {
    e.preventDefault();
    if (!store.selectedIndices.includes(index)) {
      store.selectedIndices = [index];
      store.lastClickedIndex = index;
    }
    onContextMenu(e, index);
  }

  function sortIndicator(col: SortColumn): string {
    if (store.sortColumn !== col) return "";
    return store.sortOrder === "asc" ? " ▲" : " ▼";
  }

  function getFileIcon(name: string, isDir: boolean) {
    if (isDir) return Folder;
    const ext = name.split(".").pop()?.toLowerCase() || "";

    // Images
    if (["png", "jpg", "jpeg", "gif", "svg", "webp", "bmp", "ico", "tiff", "tif", "avif", "heic", "heif", "raw", "cr2", "nef", "psd", "ai", "eps"].includes(ext)) return FileImage;

    // Video
    if (["mp4", "mkv", "avi", "mov", "webm", "wmv", "flv", "m4v", "3gp", "ogv", "ts", "m2ts", "vob"].includes(ext)) return Clapperboard;

    // Audio
    if (["mp3", "wav", "flac", "ogg", "aac", "wma", "m4a", "opus", "mid", "midi", "aiff"].includes(ext)) return Music;

    // Archives
    if (["zip", "tar", "gz", "bz2", "zst", "rar", "7z", "xz", "lz", "lzma", "cab", "iso", "dmg", "pkg", "deb", "rpm", "apk", "appimage", "snap", "flatpak"].includes(ext)) return FileArchive;

    // Databases
    if (["db", "sqlite", "sqlite3", "mdb", "accdb", "sql", "dump"].includes(ext)) return Database;

    // Spreadsheets
    if (["csv", "xlsx", "xls", "ods", "tsv", "numbers"].includes(ext)) return FileSpreadsheet;

    // Presentations
    if (["pptx", "ppt", "odp", "key"].includes(ext)) return Presentation;

    // Web
    if (["html", "htm", "xhtml", "php", "asp", "aspx", "jsp", "erb", "ejs", "svelte", "vue", "jsx", "tsx"].includes(ext)) return Globe;

    // JSON/Data
    if (["json", "jsonl", "json5", "ndjson"].includes(ext)) return FileJson;

    // Config/Settings
    if (["toml", "yaml", "yml", "ini", "cfg", "conf", "env", "properties", "plist"].includes(ext)) return FileCog;

    // Shell/Terminal
    if (["sh", "bash", "zsh", "fish", "ps1", "psm1", "bat", "cmd", "com"].includes(ext)) return FileTerminal;

    // Programming languages
    if (["rs", "go", "c", "cpp", "cc", "cxx", "h", "hpp", "hxx", "java", "kt", "kts", "scala", "clj", "cljs", "erl", "ex", "exs", "hs", "ml", "fs", "fsx", "nim", "zig", "v", "d", "r", "jl", "lua", "tcl", "perl", "pl", "pm"].includes(ext)) return FileCode;

    // Scripting languages
    if (["js", "mjs", "cjs", "ts", "mts", "py", "rb", "swift", "dart", "groovy", "coffee"].includes(ext)) return Braces;

    // CSS/Styling
    if (["css", "scss", "sass", "less", "styl", "stylus", "pcss", "postcss"].includes(ext)) return FileType;

    // Documents
    if (["pdf"].includes(ext)) return FileText;
    if (["doc", "docx", "odt", "rtf", "tex", "latex", "pages"].includes(ext)) return FileText;

    // Markdown/Text
    if (["md", "mdx", "markdown", "txt", "text", "log", "changelog", "readme", "license", "licence", "todo", "note", "notes"].includes(ext)) return FileText;

    // XML/Markup
    if (["xml", "xsl", "xslt", "xsd", "dtd", "wsdl", "rss", "atom", "soap"].includes(ext)) return FileCode;

    // Fonts
    if (["ttf", "otf", "woff", "woff2", "eot", "fnt"].includes(ext)) return FileType;

    // Security/Keys
    if (["pem", "crt", "cer", "key", "pub", "p12", "pfx", "jks", "keystore", "gpg", "pgp", "asc", "sig"].includes(ext)) return FileKey;

    // Lock files
    if (["lock", "lockb"].includes(ext) || name === "Cargo.lock" || name === "package-lock.json" || name === "yarn.lock" || name === "pnpm-lock.yaml") return FileLock;

    // Binary/Executable
    if (["exe", "msi", "bin", "elf", "out", "app", "dll", "so", "dylib", "a", "lib", "o", "obj", "class", "pyc", "pyo", "wasm"].includes(ext)) return Binary;

    // 3D/CAD
    if (["obj", "fbx", "stl", "blend", "3ds", "dae", "gltf", "glb", "usdz"].includes(ext)) return Box;

    // Game assets
    if (["unity", "unitypackage", "asset", "prefab", "scene", "umap", "uasset"].includes(ext)) return Gamepad2;

    // Design
    if (["fig", "sketch", "xd", "indd"].includes(ext)) return Pen;

    // Disk/System
    if (["img", "vhd", "vhdx", "vmdk", "qcow2", "vdi"].includes(ext)) return HardDrive;

    // Checksum/Hash
    if (["md5", "sha1", "sha256", "sha512", "checksum", "sum"].includes(ext)) return Hash;

    return File;
  }

  let entries = $derived(store.visibleEntries);

  const columns: { key: SortColumn; label: string; align: string; width: string }[] = [
    { key: "name", label: "Name", align: "left", width: "flex-1 min-w-[200px]" },
    { key: "size", label: "Size", align: "right", width: "w-[90px]" },
    { key: "packed", label: "Packed", align: "right", width: "w-[90px]" },
    { key: "ratio", label: "Ratio", align: "right", width: "w-[60px]" },
    { key: "type", label: "Type", align: "left", width: "w-[90px]" },
    { key: "modified", label: "Modified", align: "left", width: "w-[140px]" },
  ];
</script>

<div class="flex flex-col flex-1 overflow-hidden">
  <!-- Header -->
  <div class="flex items-center h-7 bg-[var(--bg-secondary)] border-b border-[var(--border)] text-[11px] font-semibold text-[var(--text-secondary)] select-none shrink-0">
    {#each columns as col}
      <button
        onclick={() => store.toggleSort(col.key)}
        class="h-full px-3 flex items-center gap-1 hover:bg-[var(--bg-hover)] hover:text-[var(--text-primary)] transition-colors cursor-pointer bg-transparent border-none text-inherit font-inherit {col.width}"
        style="justify-content: {col.align === 'right' ? 'flex-end' : 'flex-start'}"
      >
        {col.label}{sortIndicator(col.key)}
      </button>
    {/each}
  </div>

  <!-- Rows -->
  <div class="flex-1 overflow-y-auto">
    {#if entries.length === 0}
      <div class="flex items-center justify-center h-full text-[var(--text-muted)] text-sm">
        {store.searchQuery ? "No matching files found" : "This folder is empty"}
      </div>
    {:else}
      {#each entries as entry, i}
        {@const isSelected = store.selectedIndices.includes(i)}
        {@const iconColor = getFileIconColor(entry.name, entry.is_directory)}
        {@const IconComponent = getFileIcon(entry.name, entry.is_directory)}
        <button
          class="file-row {isSelected ? 'selected' : ''}"
          onclick={(e) => handleRowClick(e, i)}
          ondblclick={() => handleRowDblClick(i)}
          oncontextmenu={(e) => handleContextMenu(e, i)}
        >
          <!-- Name -->
          <div class="flex items-center gap-2 px-3 flex-1 min-w-[200px] overflow-hidden">
            <IconComponent size={16} color={iconColor} class="shrink-0" />
            <span class="truncate" style="color: {entry.is_directory ? 'var(--dir-color)' : 'var(--text-primary)'}">{entry.name}</span>
          </div>
          <!-- Size -->
          <div class="w-[90px] px-3 text-right text-[var(--text-secondary)]">
            {entry.is_directory ? "" : formatSize(entry.uncompressed_size)}
          </div>
          <!-- Packed -->
          <div class="w-[90px] px-3 text-right text-[var(--text-secondary)]">
            {entry.is_directory ? "" : formatSize(entry.compressed_size)}
          </div>
          <!-- Ratio -->
          <div class="w-[60px] px-3 text-right text-[var(--text-secondary)]">
            {entry.is_directory ? "" : formatRatio(entry.compressed_size, entry.uncompressed_size)}
          </div>
          <!-- Type -->
          <div class="w-[90px] px-3 text-[var(--text-secondary)] truncate">
            {fileTypeDisplay(entry.name, entry.is_directory)}
          </div>
          <!-- Modified -->
          <div class="w-[140px] px-3 text-[var(--text-muted)]">
            {entry.modified || "—"}
          </div>
        </button>
      {/each}
    {/if}
  </div>
</div>

<style>
  .file-row {
    display: flex;
    align-items: center;
    width: 100%;
    height: 28px;
    font-size: 12px;
    border: none;
    background: transparent;
    color: inherit;
    cursor: default;
    text-align: left;
    transition: background-color 0.08s;
    border-left: 2px solid transparent;
  }
  .file-row:nth-child(even) {
    background: rgba(255, 255, 255, 0.015);
  }
  .file-row:hover {
    background: var(--bg-hover);
  }
  .file-row.selected {
    background: var(--bg-active);
    border-left-color: var(--accent);
    color: white;
  }
  .file-row.selected:hover {
    background: var(--selected-hover);
  }
  .file-row.selected div {
    color: rgba(255, 255, 255, 0.85) !important;
  }
</style>
