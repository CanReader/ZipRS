export function formatSize(bytes: number): string {
  if (bytes === 0) return "—";
  const KB = 1024;
  const MB = KB * 1024;
  const GB = MB * 1024;
  if (bytes >= GB) return `${(bytes / GB).toFixed(1)} GB`;
  if (bytes >= MB) return `${(bytes / MB).toFixed(1)} MB`;
  if (bytes >= KB) return `${(bytes / KB).toFixed(1)} KB`;
  return `${bytes} B`;
}

export function formatRatio(compressed: number, uncompressed: number): string {
  if (uncompressed === 0) return "";
  const ratio = (1.0 - compressed / uncompressed) * 100.0;
  return ratio < 0 ? "0%" : `${Math.round(ratio)}%`;
}

export function fileExtension(name: string): string {
  const parts = name.split(".");
  return parts.length > 1 ? parts[parts.length - 1].toUpperCase() : "";
}

export function fileTypeDisplay(name: string, isDir: boolean): string {
  if (isDir) return "Folder";
  const ext = fileExtension(name);
  return ext ? `${ext} File` : "File";
}

export function getFileIconColor(name: string, isDir: boolean): string {
  if (isDir) return getComputedStyle(document.documentElement).getPropertyValue("--dir-color").trim() || "#4a9eff";
  const ext = fileExtension(name).toLowerCase();
  const colors: Record<string, string> = {
    // Images
    png: "#9b59b6", jpg: "#9b59b6", jpeg: "#9b59b6", gif: "#9b59b6", svg: "#e88fd0",
    webp: "#9b59b6", bmp: "#9b59b6", ico: "#9b59b6", tiff: "#9b59b6", tif: "#9b59b6",
    avif: "#9b59b6", heic: "#9b59b6", heif: "#9b59b6", raw: "#9b59b6", cr2: "#9b59b6",
    nef: "#9b59b6", psd: "#31a8ff", ai: "#ff9a00", eps: "#ff9a00",

    // Video
    mp4: "#e25555", mkv: "#e25555", avi: "#e25555", mov: "#e25555", webm: "#e25555",
    wmv: "#e25555", flv: "#e25555", m4v: "#e25555", "3gp": "#e25555", ogv: "#e25555",
    ts: "#3178c6", m2ts: "#e25555", vob: "#e25555",

    // Audio
    mp3: "#1db954", wav: "#1db954", flac: "#f5a623", ogg: "#1db954", aac: "#1db954",
    wma: "#1db954", m4a: "#1db954", opus: "#1db954", mid: "#8a6dba", midi: "#8a6dba",
    aiff: "#1db954",

    // Archives
    zip: "#e2a94a", tar: "#e2a94a", gz: "#e2a94a", bz2: "#e2a94a", zst: "#e2a94a",
    rar: "#e2a94a", "7z": "#e2a94a", xz: "#e2a94a", lz: "#e2a94a", lzma: "#e2a94a",
    cab: "#e2a94a", iso: "#c0c0c0", dmg: "#c0c0c0", deb: "#a93226", rpm: "#cc0000",
    apk: "#3ddc84", appimage: "#1a9fff", snap: "#e95420", flatpak: "#4a86cf",
    pkg: "#c0c0c0",

    // Databases
    db: "#336791", sqlite: "#336791", sqlite3: "#336791", mdb: "#336791",
    accdb: "#336791", sql: "#e38c00", dump: "#336791",

    // Spreadsheets
    csv: "#4abb8a", xlsx: "#217346", xls: "#217346", ods: "#4abb8a",
    tsv: "#4abb8a", numbers: "#4abb8a",

    // Presentations
    pptx: "#d04423", ppt: "#d04423", odp: "#d04423", key: "#009dff",

    // Web
    html: "#e34c26", htm: "#e34c26", xhtml: "#e34c26", php: "#777bb3",
    asp: "#6a40fd", aspx: "#6a40fd", jsp: "#e34c26", erb: "#cc342d",
    ejs: "#a0c84e", svelte: "#ff3e00", vue: "#42b883", jsx: "#61dafb", tsx: "#3178c6",

    // JSON/Data
    json: "#f5a623", jsonl: "#f5a623", json5: "#f5a623", ndjson: "#f5a623",

    // Config
    toml: "#9c4121", yaml: "#cb171e", yml: "#cb171e", ini: "#a0a2b0",
    cfg: "#a0a2b0", conf: "#a0a2b0", env: "#ecd53f", properties: "#a0a2b0",
    plist: "#a0a2b0",

    // Shell/Terminal
    sh: "#4abb8a", bash: "#4abb8a", zsh: "#4abb8a", fish: "#4abb8a",
    ps1: "#012456", psm1: "#012456", bat: "#40826d", cmd: "#40826d", com: "#40826d",

    // Rust
    rs: "#dea584",

    // Go
    go: "#00add8",

    // C/C++
    c: "#555555", cpp: "#f34b7d", cc: "#f34b7d", cxx: "#f34b7d",
    h: "#555555", hpp: "#f34b7d", hxx: "#f34b7d",

    // Java/JVM
    java: "#b07219", kt: "#a97bff", kts: "#a97bff", scala: "#c22d40",
    clj: "#db5855", cljs: "#db5855", groovy: "#4298b8",

    // Functional
    erl: "#b83998", ex: "#6e4a7e", exs: "#6e4a7e",
    hs: "#5e5086", ml: "#dc566d", fs: "#b845fc", fsx: "#b845fc",

    // Other langs
    nim: "#ffc200", zig: "#f7a41d", v: "#5d87bf", d: "#b03931",
    r: "#198ce7", jl: "#9558b2", lua: "#000080", tcl: "#e4cc98",
    perl: "#0298c3", pl: "#0298c3", pm: "#0298c3",

    // Scripting
    js: "#f0db4f", mjs: "#f0db4f", cjs: "#f0db4f",
    ts: "#3178c6", mts: "#3178c6",
    py: "#3572a5", rb: "#cc342d", swift: "#f05138", dart: "#00b4ab",
    coffee: "#244776",

    // CSS
    css: "#264de4", scss: "#cd6799", sass: "#cd6799", less: "#1d365d",
    styl: "#ff6347", stylus: "#ff6347", pcss: "#dd3a0a", postcss: "#dd3a0a",

    // Documents
    pdf: "#e25555", doc: "#2b579a", docx: "#2b579a", odt: "#2b579a",
    rtf: "#2b579a", tex: "#3d6117", latex: "#3d6117", pages: "#ff9500",

    // Markdown/Text
    md: "#519aba", mdx: "#fcb32c", markdown: "#519aba",
    txt: "#a0a2b0", text: "#a0a2b0", log: "#888888",
    changelog: "#a0a2b0", readme: "#519aba", license: "#d4a520",
    licence: "#d4a520", todo: "#a0a2b0",

    // XML
    xml: "#e65100", xsl: "#e65100", xslt: "#e65100", xsd: "#e65100",
    dtd: "#e65100", wsdl: "#e65100", rss: "#ee802f", atom: "#ee802f",

    // Fonts
    ttf: "#e88fd0", otf: "#e88fd0", woff: "#e88fd0", woff2: "#e88fd0",
    eot: "#e88fd0", fnt: "#e88fd0",

    // Security/Keys
    pem: "#c94040", crt: "#c94040", cer: "#c94040", key: "#c94040",
    pub: "#4abb8a", p12: "#c94040", pfx: "#c94040", jks: "#c94040",
    keystore: "#c94040", gpg: "#c94040", pgp: "#c94040", asc: "#c94040",
    sig: "#c94040",

    // Lock files
    lock: "#888888", lockb: "#888888",

    // Binary/Executable
    exe: "#e25555", msi: "#e25555", bin: "#888888", elf: "#888888",
    out: "#888888", app: "#888888", dll: "#8a6dba", so: "#8a6dba",
    dylib: "#8a6dba", a: "#888888", lib: "#888888", o: "#888888",
    obj: "#888888", class: "#b07219", pyc: "#3572a5", pyo: "#3572a5",
    wasm: "#654ff0",

    // 3D/CAD
    fbx: "#0e7a0e", stl: "#0e7a0e", blend: "#e87d0d", "3ds": "#0e7a0e",
    dae: "#0e7a0e", gltf: "#5cb85c", glb: "#5cb85c", usdz: "#5cb85c",

    // Game
    unity: "#222c37", unitypackage: "#222c37", asset: "#222c37",
    prefab: "#222c37", scene: "#222c37", umap: "#222c37", uasset: "#222c37",

    // Design
    fig: "#a259ff", sketch: "#fdad00", xd: "#ff61f6", indd: "#ff3366",

    // Disk images
    img: "#c0c0c0", vhd: "#c0c0c0", vhdx: "#c0c0c0", vmdk: "#c0c0c0",
    qcow2: "#c0c0c0", vdi: "#c0c0c0",

    // Checksums
    md5: "#888888", sha1: "#888888", sha256: "#888888", sha512: "#888888",
    checksum: "#888888", sum: "#888888",
  };
  return colors[ext] || "#8a8c9a";
}
