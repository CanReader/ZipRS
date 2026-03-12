import type { ArchiveEntry, SortColumn, SortOrder, ProgressPayload } from "./types";

class AppStore {
  // Archive
  archivePath = $state<string | null>(null);
  archiveFormat = $state<string | null>(null);
  allEntries = $state<ArchiveEntry[]>([]);
  supportsModification = $state(false);

  // Navigation
  currentPath = $state("");
  pathHistory = $state<string[]>([""]);
  historyIndex = $state(0);

  // Selection
  selectedIndices = $state<number[]>([]);
  lastClickedIndex = $state<number | null>(null);

  // Sort
  sortColumn = $state<SortColumn>("name");
  sortOrder = $state<SortOrder>("asc");

  // Search
  searchQuery = $state("");

  // UI
  isBusy = $state(false);
  progress = $state<ProgressPayload>({ current: 0, total: 0, message: "" });
  statusMessage = $state("Ready");
  errorMessage = $state<string | null>(null);
  infoMessage = $state<string | null>(null);
  showAbout = $state(false);
  showProperties = $state(false);
  showCreateDialog = $state(false);

  // Derived
  get hasArchive() { return this.archivePath !== null; }
  get hasSelection() { return this.selectedIndices.length > 0; }
  get canModify() { return this.supportsModification; }
  get canGoBack() { return this.historyIndex > 0; }
  get canGoForward() { return this.historyIndex + 1 < this.pathHistory.length; }
  get canGoUp() { return this.currentPath !== ""; }

  get visibleEntries(): ArchiveEntry[] {
    let entries = this.allEntries.filter((e) => {
      const parent = e.path.includes("/")
        ? e.path.substring(0, e.path.lastIndexOf("/"))
        : "";
      return parent === this.currentPath;
    });

    if (this.searchQuery) {
      const q = this.searchQuery.toLowerCase();
      entries = entries.filter((e) => e.name.toLowerCase().includes(q));
    }

    entries.sort((a, b) => {
      // Directories first
      if (a.is_directory && !b.is_directory) return -1;
      if (!a.is_directory && b.is_directory) return 1;

      let cmp = 0;
      switch (this.sortColumn) {
        case "name": cmp = a.name.localeCompare(b.name, undefined, { sensitivity: "base" }); break;
        case "size": cmp = a.uncompressed_size - b.uncompressed_size; break;
        case "packed": cmp = a.compressed_size - b.compressed_size; break;
        case "ratio": cmp = a.compressed_size - b.compressed_size; break;
        case "type": cmp = (a.name.split(".").pop() || "").localeCompare(b.name.split(".").pop() || ""); break;
        case "modified": cmp = (a.modified || "").localeCompare(b.modified || ""); break;
      }
      return this.sortOrder === "asc" ? cmp : -cmp;
    });

    return entries;
  }

  get selectedEntries(): ArchiveEntry[] {
    const visible = this.visibleEntries;
    return this.selectedIndices
      .filter((i) => i < visible.length)
      .map((i) => visible[i]);
  }

  // Actions
  openArchiveResult(path: string, entries: ArchiveEntry[], format: string, supportsMod: boolean) {
    this.archivePath = path;
    this.archiveFormat = format;
    this.allEntries = entries;
    this.supportsModification = supportsMod;
    this.currentPath = "";
    this.pathHistory = [""];
    this.historyIndex = 0;
    this.selectedIndices = [];
    this.lastClickedIndex = null;
    this.searchQuery = "";
    this.isBusy = false;
    this.statusMessage = `${entries.length} entries loaded`;
  }

  navigateTo(path: string) {
    if (path !== this.currentPath) {
      this.pathHistory = this.pathHistory.slice(0, this.historyIndex + 1);
      this.pathHistory.push(path);
      this.historyIndex = this.pathHistory.length - 1;
      this.currentPath = path;
      this.selectedIndices = [];
      this.lastClickedIndex = null;
    }
  }

  navigateBack() {
    if (this.historyIndex > 0) {
      this.historyIndex--;
      this.currentPath = this.pathHistory[this.historyIndex];
      this.selectedIndices = [];
    }
  }

  navigateForward() {
    if (this.historyIndex + 1 < this.pathHistory.length) {
      this.historyIndex++;
      this.currentPath = this.pathHistory[this.historyIndex];
      this.selectedIndices = [];
    }
  }

  navigateUp() {
    if (this.currentPath) {
      const idx = this.currentPath.lastIndexOf("/");
      const parent = idx >= 0 ? this.currentPath.substring(0, idx) : "";
      this.navigateTo(parent);
    }
  }

  selectAll() {
    this.selectedIndices = Array.from({ length: this.visibleEntries.length }, (_, i) => i);
  }

  reset() {
    this.archivePath = null;
    this.archiveFormat = null;
    this.allEntries = [];
    this.supportsModification = false;
    this.currentPath = "";
    this.pathHistory = [""];
    this.historyIndex = 0;
    this.selectedIndices = [];
    this.lastClickedIndex = null;
    this.searchQuery = "";
    this.isBusy = false;
    this.statusMessage = "Ready";
  }

  toggleSort(column: SortColumn) {
    if (this.sortColumn === column) {
      this.sortOrder = this.sortOrder === "asc" ? "desc" : "asc";
    } else {
      this.sortColumn = column;
      this.sortOrder = "asc";
    }
  }
}

export const store = new AppStore();
