<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { open as dialogOpen, save as dialogSave } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";

  import { store } from "$lib/store.svelte";
  import { loadSavedTheme } from "$lib/themes";
  import type { OpenArchiveResponse, ProgressPayload } from "$lib/types";
  import WelcomeScreen from "$lib/components/WelcomeScreen.svelte";
  import Toolbar from "$lib/components/Toolbar.svelte";
  import AddressBar from "$lib/components/AddressBar.svelte";
  import FileTable from "$lib/components/FileTable.svelte";
  import StatusBar from "$lib/components/StatusBar.svelte";
  import ContextMenu from "$lib/components/ContextMenu.svelte";
  import ProgressDialog from "$lib/components/ProgressDialog.svelte";
  import Dialogs from "$lib/components/Dialogs.svelte";

  // Context menu state
  let ctxVisible = $state(false);
  let ctxX = $state(0);
  let ctxY = $state(0);

  // Drop overlay
  let showDropOverlay = $state(false);

  function handleDrop(paths: string[]) {
    if (paths.length === 1) {
      handleOpenArchive(paths[0]);
    } else if (paths.length > 1 && store.hasArchive && store.canModify) {
      handleAddFiles(paths);
    }
  }

  onMount(async () => {
    // Load saved theme
    store.theme = loadSavedTheme();

    // Listen for progress events
    await listen<ProgressPayload>("archive-progress", (event) => {
      store.progress = event.payload;
    });

    // Listen for file drop via Tauri's webview drag-drop API
    const appWindow = getCurrentWebviewWindow();
    await appWindow.onDragDropEvent((event) => {
      if (event.payload.type === "over") {
        showDropOverlay = true;
      } else if (event.payload.type === "drop") {
        showDropOverlay = false;
        handleDrop(event.payload.paths);
      } else {
        showDropOverlay = false;
      }
    });

    // Also listen via generic event (fallback for some platforms)
    await listen<{ paths: string[] }>("tauri://drag-drop", (event) => {
      showDropOverlay = false;
      if (event.payload.paths?.length > 0) {
        handleDrop(event.payload.paths);
      }
    });
    await listen("tauri://drag-over", () => {
      showDropOverlay = true;
    });
    await listen("tauri://drag-leave", () => {
      showDropOverlay = false;
    });

    // Handle file association opens (double-click archive in file manager)
    await listen<string>("open-file-association", (event) => {
      handleOpenArchive(event.payload);
    });
  });

  // ---- Actions ----
  async function handleAction(action: string) {
    switch (action) {
      case "open": await handleOpenDialog(); break;
      case "create": await handleCreateDialog(); break;
      case "add": await handleAddDialog(); break;
      case "extract": await handleExtractAll(); break;
      case "extract-here": await handleExtractHere(); break;
      case "extract-selected": await handleExtractSelected(); break;
      case "delete": await handleDelete(); break;
      case "test": await handleTest(); break;
      case "open-file": await handleOpenFile(); break;
      case "drag-out": await handleDragOut(); break;
      case "properties": store.showProperties = true; break;
      case "about": store.showAbout = true; break;
      case "refresh": await handleRefresh(); break;
      case "back": store.navigateBack(); break;
      case "forward": store.navigateForward(); break;
      case "up": store.navigateUp(); break;
      case "select-all": store.selectAll(); break;
    }
  }

  async function handleOpenDialog() {
    const path = await dialogOpen({
      title: "Open Archive",
      filters: [
        { name: "Archives", extensions: ["zip", "rar", "tar", "gz", "tgz", "bz2", "tbz2", "zst", "tzst", "jar", "war"] },
        { name: "ZIP files", extensions: ["zip", "jar", "war"] },
        { name: "RAR files", extensions: ["rar"] },
        { name: "TAR files", extensions: ["tar", "gz", "tgz", "bz2", "tbz2", "zst"] },
        { name: "All files", extensions: ["*"] },
      ],
    });
    if (path) await handleOpenArchive(path as string);
  }

  async function handleOpenArchive(path: string) {
    try {
      store.isBusy = true;
      store.statusMessage = "Opening archive...";
      const result = await invoke<OpenArchiveResponse>("open_archive", { path });
      store.openArchiveResult(result.path, result.entries, result.format, result.supports_modification);
    } catch (e) {
      store.isBusy = false;
      store.errorMessage = String(e);
      store.statusMessage = "Error occurred";
    }
  }

  async function handleExtractAll() {
    const dest = await dialogOpen({ title: "Extract All To...", directory: true });
    if (!dest) return;
    try {
      store.isBusy = true;
      store.statusMessage = "Extracting...";
      const msg = await invoke<string>("extract_all", { dest: dest as string });
      store.isBusy = false;
      store.infoMessage = msg;
      store.statusMessage = msg;
    } catch (e) {
      store.isBusy = false;
      store.errorMessage = String(e);
    }
  }

  async function handleExtractHere() {
    try {
      store.isBusy = true;
      store.statusMessage = "Extracting here...";
      const msg = await invoke<string>("extract_here");
      store.isBusy = false;
      store.infoMessage = msg;
      store.statusMessage = msg;
    } catch (e) {
      store.isBusy = false;
      store.errorMessage = String(e);
    }
  }

  async function handleExtractSelected() {
    const entries = store.selectedEntries.map((e) => e.path);
    if (entries.length === 0) return;
    const dest = await dialogOpen({ title: "Extract Selected To...", directory: true });
    if (!dest) return;
    try {
      store.isBusy = true;
      store.statusMessage = "Extracting...";
      const msg = await invoke<string>("extract_selected", { entries, dest: dest as string });
      store.isBusy = false;
      store.infoMessage = msg;
      store.statusMessage = msg;
    } catch (e) {
      store.isBusy = false;
      store.errorMessage = String(e);
    }
  }

  async function handleAddDialog() {
    const files = await dialogOpen({ title: "Add Files to Archive", multiple: true });
    if (files && (files as string[]).length > 0) {
      await handleAddFiles(files as string[]);
    }
  }

  async function handleAddFiles(files: string[]) {
    try {
      store.isBusy = true;
      store.statusMessage = "Adding files...";
      const result = await invoke<OpenArchiveResponse>("add_files", {
        files,
        archivePrefix: store.currentPath,
      });
      store.allEntries = result.entries;
      store.isBusy = false;
      store.infoMessage = `Added ${files.length} file(s)`;
      store.statusMessage = `${result.entries.length} entries loaded`;
    } catch (e) {
      store.isBusy = false;
      store.errorMessage = String(e);
    }
  }

  async function handleDelete() {
    const entries = store.selectedEntries.map((e) => e.path);
    if (entries.length === 0) return;
    try {
      store.isBusy = true;
      store.statusMessage = "Deleting...";
      const result = await invoke<OpenArchiveResponse>("delete_entries", { entries });
      store.allEntries = result.entries;
      store.selectedIndices = [];
      store.isBusy = false;
      store.statusMessage = `${result.entries.length} entries loaded`;
    } catch (e) {
      store.isBusy = false;
      store.errorMessage = String(e);
    }
  }

  async function handleTest() {
    try {
      store.isBusy = true;
      store.statusMessage = "Testing archive...";
      const msg = await invoke<string>("test_archive");
      store.isBusy = false;
      store.infoMessage = msg;
      store.statusMessage = msg;
    } catch (e) {
      store.isBusy = false;
      store.errorMessage = String(e);
    }
  }

  async function handleOpenFile() {
    const selected = store.selectedEntries;
    if (selected.length === 0) return;
    const entry = selected[0];
    if (entry.is_directory) {
      store.navigateTo(entry.path);
    } else {
      try {
        store.statusMessage = "Extracting file...";
        await invoke("extract_and_open", { entryPath: entry.path });
        store.statusMessage = `Opened: ${entry.name}`;
      } catch (e) {
        store.errorMessage = String(e);
      }
    }
  }

  async function handleDragOut() {
    const selected = store.selectedEntries;
    if (selected.length === 0) return;
    const entryPaths = selected.map((e) => e.path);
    try {
      store.statusMessage = "Extracting files for drag...";
      const filePaths = await invoke<string[]>("drag_out", { entries: entryPaths });
      store.statusMessage = "Dragging — drop files to destination";
      const { startDrag } = await import("@crabnebula/tauri-plugin-drag");
      await startDrag({ item: filePaths, icon: "" });
      store.statusMessage = "Files dragged successfully";
    } catch (e) {
      store.errorMessage = String(e);
    }
  }

  async function handleRefresh() {
    if (!store.archivePath) return;
    await handleOpenArchive(store.archivePath);
  }

  async function handleCreateDialog() {
    // For now simple: pick files then save location
    const files = await dialogOpen({ title: "Select files to archive", multiple: true });
    if (!files || (files as string[]).length === 0) return;

    const savePath = await dialogSave({
      title: "Save archive as",
      defaultPath: "archive.zip",
      filters: [
        { name: "ZIP", extensions: ["zip"] },
        { name: "TAR.GZ", extensions: ["tar.gz"] },
        { name: "TAR.BZ2", extensions: ["tar.bz2"] },
        { name: "TAR.ZST", extensions: ["tar.zst"] },
        { name: "TAR", extensions: ["tar"] },
      ],
    });
    if (!savePath) return;

    // Detect format from extension
    const path = savePath as string;
    let format = "Zip";
    if (path.endsWith(".tar.gz") || path.endsWith(".tgz")) format = "TarGz";
    else if (path.endsWith(".tar.bz2")) format = "TarBz2";
    else if (path.endsWith(".tar.zst")) format = "TarZst";
    else if (path.endsWith(".tar")) format = "Tar";

    const fileList = files as string[];
    const baseDir = fileList[0].substring(0, fileList[0].lastIndexOf("/")) || "/";

    try {
      store.isBusy = true;
      store.statusMessage = "Creating archive...";
      const createdPath = await invoke<string>("create_archive", {
        path, files: fileList, baseDir, format,
      });
      store.isBusy = false;
      store.statusMessage = "Archive created";
      await handleOpenArchive(createdPath);
    } catch (e) {
      store.isBusy = false;
      store.errorMessage = String(e);
    }
  }

  function handleContextMenu(e: MouseEvent, _index: number) {
    ctxX = e.clientX;
    ctxY = e.clientY;
    ctxVisible = true;
  }

  // Keyboard shortcuts
  function handleKeydown(e: KeyboardEvent) {
    // Block dev tools shortcuts
    if (e.key === "F12") { e.preventDefault(); return; }
    if ((e.ctrlKey || e.metaKey) && e.shiftKey && ["i", "c", "j"].includes(e.key.toLowerCase())) { e.preventDefault(); return; }
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "u") { e.preventDefault(); return; }

    if (e.ctrlKey || e.metaKey) {
      switch (e.key.toLowerCase()) {
        case "o": e.preventDefault(); handleAction("open"); break;
        case "n": e.preventDefault(); handleAction("create"); break;
        case "e": e.preventDefault(); handleAction("extract"); break;
        case "h": e.preventDefault(); handleAction("extract-here"); break;
        case "t": e.preventDefault(); handleAction("test"); break;
        case "a": e.preventDefault(); handleAction("select-all"); break;
      }
    } else if (e.altKey) {
      switch (e.key) {
        case "ArrowLeft": e.preventDefault(); handleAction("back"); break;
        case "ArrowRight": e.preventDefault(); handleAction("forward"); break;
        case "Enter": e.preventDefault(); handleAction("properties"); break;
      }
    } else {
      switch (e.key) {
        case "Delete": handleAction("delete"); break;
        case "Enter": handleAction("open-file"); break;
        case "Backspace": handleAction("up"); break;
        case "F5": e.preventDefault(); handleAction("refresh"); break;
      }
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />
<!-- Block browser context menu globally -->
<svelte:document oncontextmenu={(e) => {
  // Only allow our custom context menu from file rows
  const target = e.target as HTMLElement;
  if (!target.closest('.file-row')) {
    e.preventDefault();
  }
}} />

<div class="flex flex-col h-screen">
  {#if store.hasArchive}
    <Toolbar onAction={handleAction} />
    <AddressBar />
    <FileTable onAction={handleAction} onContextMenu={handleContextMenu} />
    <StatusBar />
  {:else}
    <Toolbar onAction={handleAction} />
    <WelcomeScreen onOpen={() => handleAction("open")} onCreate={() => handleAction("create")} />
    <StatusBar />
  {/if}
</div>

<!-- Overlays -->
<ContextMenu x={ctxX} y={ctxY} visible={ctxVisible} onAction={handleAction} onClose={() => ctxVisible = false} />
<ProgressDialog />
<Dialogs onAction={handleAction} />

<!-- Drop overlay -->
{#if showDropOverlay}
  <div class="fixed inset-0 z-40 flex items-center justify-center bg-blue-900/30 backdrop-blur-sm border-2 border-dashed border-blue-400/50 pointer-events-none">
    <div class="text-center">
      <p class="text-xl font-semibold text-blue-300">Drop to open or add files</p>
      <p class="text-sm text-blue-400/60 mt-1">Release to proceed</p>
    </div>
  </div>
{/if}
