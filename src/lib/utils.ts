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
    // Documents
    pdf: "#e25555", doc: "#4a90e2", docx: "#4a90e2", txt: "#a0a2b0",
    // Images
    png: "#9b59b6", jpg: "#9b59b6", jpeg: "#9b59b6", gif: "#9b59b6", svg: "#9b59b6", webp: "#9b59b6",
    // Code
    js: "#f0db4f", ts: "#3178c6", rs: "#dea584", py: "#3572a5", html: "#e34c26", css: "#264de4",
    // Archives
    zip: "#e2a94a", tar: "#e2a94a", gz: "#e2a94a", bz2: "#e2a94a", zst: "#e2a94a",
    // Audio/Video
    mp3: "#1db954", mp4: "#e25555", mkv: "#e25555", avi: "#e25555",
    // Data
    json: "#a0a2b0", xml: "#a0a2b0", csv: "#4abb8a", xlsx: "#4abb8a",
    // Executables
    exe: "#e25555", sh: "#4abb8a", bat: "#4abb8a",
  };
  return colors[ext] || "#8a8c9a";
}
