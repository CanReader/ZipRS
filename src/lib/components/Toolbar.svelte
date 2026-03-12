<script lang="ts">
  import {
    ArrowLeft, ArrowRight, ArrowUp, FolderOpen, Plus,
    Download, FolderDown, ShieldCheck, Trash2, Info,
    FilePlus2, RefreshCw
  } from "lucide-svelte";
  import { store } from "$lib/store.svelte";

  interface Props {
    onAction: (action: string) => void;
  }

  let { onAction }: Props = $props();
</script>

<div class="toolbar">
  <!-- Navigation -->
  <div class="btn-group">
    <button onclick={() => onAction("back")} disabled={!store.canGoBack || store.isBusy}
      class="tbtn" title="Back (Alt+Left)">
      <ArrowLeft size={17} strokeWidth={2} />
    </button>
    <button onclick={() => onAction("forward")} disabled={!store.canGoForward || store.isBusy}
      class="tbtn" title="Forward (Alt+Right)">
      <ArrowRight size={17} strokeWidth={2} />
    </button>
    <button onclick={() => onAction("up")} disabled={!store.canGoUp || store.isBusy}
      class="tbtn" title="Up (Backspace)">
      <ArrowUp size={17} strokeWidth={2} />
    </button>
  </div>

  <div class="sep"></div>

  <!-- Main actions -->
  <button onclick={() => onAction("open")} disabled={store.isBusy}
    class="tbtn with-label" title="Open archive (Ctrl+O)">
    <FolderOpen size={17} strokeWidth={1.8} class="text-blue-400" />
    <span>Open</span>
  </button>

  <button onclick={() => onAction("add")} disabled={!store.hasArchive || !store.canModify || store.isBusy}
    class="tbtn with-label" title="Add files to archive">
    <Plus size={17} strokeWidth={1.8} class="text-green-400" />
    <span>Add</span>
  </button>

  <button onclick={() => onAction("extract")} disabled={!store.hasArchive || store.isBusy}
    class="tbtn with-label" title="Extract all (Ctrl+E)">
    <Download size={17} strokeWidth={1.8} class="text-cyan-400" />
    <span>Extract</span>
  </button>

  <button onclick={() => onAction("extract-here")} disabled={!store.hasArchive || store.isBusy}
    class="tbtn with-label" title="Extract here (Ctrl+H)">
    <FolderDown size={17} strokeWidth={1.8} class="text-cyan-300" />
    <span>Here</span>
  </button>

  <div class="sep"></div>

  <button onclick={() => onAction("test")} disabled={!store.hasArchive || store.isBusy}
    class="tbtn with-label" title="Test archive (Ctrl+T)">
    <ShieldCheck size={17} strokeWidth={1.8} class="text-emerald-400" />
    <span>Test</span>
  </button>

  <button onclick={() => onAction("delete")} disabled={!store.hasSelection || !store.canModify || store.isBusy}
    class="tbtn with-label" title="Delete selected (Del)">
    <Trash2 size={17} strokeWidth={1.8} class="text-red-400" />
    <span>Delete</span>
  </button>

  <div class="sep"></div>

  <button onclick={() => onAction("properties")} disabled={!store.hasSelection || store.isBusy}
    class="tbtn with-label" title="Properties (Alt+Enter)">
    <Info size={17} strokeWidth={1.8} class="text-purple-400" />
    <span>Info</span>
  </button>

  <button onclick={() => onAction("create")} disabled={store.isBusy}
    class="tbtn with-label" title="Create archive (Ctrl+N)">
    <FilePlus2 size={17} strokeWidth={1.8} class="text-amber-400" />
    <span>New</span>
  </button>

  <div class="spacer"></div>

  <button onclick={() => onAction("refresh")} disabled={!store.hasArchive || store.isBusy}
    class="tbtn" title="Refresh (F5)">
    <RefreshCw size={15} strokeWidth={2} />
  </button>
</div>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    height: 44px;
    padding: 0 8px;
    gap: 2px;
    background: #222327;
    border-bottom: 1px solid #2e2f36;
  }

  .btn-group {
    display: flex;
    gap: 1px;
  }

  .tbtn {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 34px;
    min-width: 34px;
    padding: 0 8px;
    border-radius: 8px;
    border: none;
    background: transparent;
    color: #a0a2b0;
    cursor: pointer;
    transition: all 0.15s;
  }

  .tbtn:hover:not(:disabled) {
    background: #32333a;
    color: #e8e9ed;
  }

  .tbtn:active:not(:disabled) {
    background: #3a3b44;
  }

  .tbtn:disabled {
    opacity: 0.25;
    cursor: default;
  }

  .with-label {
    gap: 6px;
    padding: 0 12px;
    font-size: 12px;
    font-weight: 500;
  }

  .sep {
    width: 1px;
    height: 22px;
    background: #35363d;
    margin: 0 6px;
    flex-shrink: 0;
  }

  .spacer {
    flex: 1;
  }
</style>
