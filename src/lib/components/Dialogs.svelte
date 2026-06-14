<script lang="ts">
  import { store } from "$lib/store.svelte";
  import { formatSize, formatRatio, fileTypeDisplay } from "$lib/utils";
  import { X, AlertCircle, CheckCircle, Folder, File } from "lucide-svelte";

  interface Props {
    onAction: (action: string) => void;
  }

  let { onAction }: Props = $props();
</script>

<!-- Error Dialog -->
{#if store.errorMessage}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
    onclick={(e) => { if (e.target === e.currentTarget) store.errorMessage = null; }}
    onkeydown={(e) => { if (e.key === "Escape") store.errorMessage = null; }}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div class="dialog-card w-[440px]">
      <div class="dialog-header border-b border-[var(--border)] bg-red-500/5">
        <div class="icon-badge bg-red-500/15">
          <AlertCircle size={16} class="text-red-400" />
        </div>
        <span class="dialog-title text-red-400">Error</span>
        <button class="close-btn" onclick={() => store.errorMessage = null}>
          <X size={14} />
        </button>
      </div>
      <div class="dialog-body">
        <p class="text-sm text-[var(--text-secondary)] leading-relaxed">{store.errorMessage}</p>
      </div>
      <div class="dialog-footer">
        <button
          class="btn-full bg-red-500/12 hover:bg-red-500/22 text-red-400 border border-red-500/20"
          onclick={() => store.errorMessage = null}
        >
          Dismiss
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Info / Success Dialog -->
{#if store.infoMessage}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
    onclick={(e) => { if (e.target === e.currentTarget) store.infoMessage = null; }}
    onkeydown={(e) => { if (e.key === "Escape") store.infoMessage = null; }}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div class="dialog-card w-[440px]">
      <div class="dialog-header border-b border-[var(--border)] bg-emerald-500/5">
        <div class="icon-badge bg-emerald-500/15">
          <CheckCircle size={16} class="text-emerald-400" />
        </div>
        <span class="dialog-title text-emerald-400">Done</span>
        <button class="close-btn" onclick={() => store.infoMessage = null}>
          <X size={14} />
        </button>
      </div>
      <div class="dialog-body">
        <p class="text-sm text-[var(--text-secondary)] leading-relaxed">{store.infoMessage}</p>
      </div>
      <div class="dialog-footer">
        <button
          class="btn-full bg-[var(--accent)] hover:bg-[var(--accent-hover)] text-white border-none"
          onclick={() => store.infoMessage = null}
        >
          Got it
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- About Dialog -->
{#if store.showAbout}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
    onclick={(e) => { if (e.target === e.currentTarget) store.showAbout = false; }}
    onkeydown={(e) => { if (e.key === "Escape") store.showAbout = false; }}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div class="dialog-card w-[380px]">
      <div class="relative flex flex-col items-center pt-8 pb-6 px-6 bg-gradient-to-b from-[var(--accent)]/10 to-transparent border-b border-[var(--border)]">
        <button class="close-btn absolute top-3 right-3" onclick={() => store.showAbout = false}>
          <X size={14} />
        </button>
        <img src="/Logo.png" alt="ZipRS" class="w-16 h-16 mb-3 drop-shadow-lg" />
        <h2 class="text-xl font-bold bg-gradient-to-r from-blue-400 to-cyan-400 bg-clip-text text-transparent">ZipRS</h2>
        <p class="text-[11px] text-[var(--text-muted)] mt-0.5 tracking-wide">Archive Manager</p>
      </div>

      <div class="px-6 py-5 space-y-3">
        <div class="about-row">
          <span class="about-label">Version</span>
          <span class="px-2 py-0.5 rounded-md bg-[var(--bg-tertiary)] border border-[var(--border)] text-[var(--text-secondary)] font-mono text-[11px]">0.1.0</span>
        </div>
        <div class="about-row">
          <span class="about-label">Formats</span>
          <div class="flex flex-wrap gap-1 justify-end">
            {#each ["ZIP", "RAR", "TAR", "TAR.GZ", "TAR.BZ2", "TAR.ZST"] as fmt}
              <span class="px-1.5 py-0.5 rounded bg-[var(--accent)]/10 text-[var(--accent)] text-[10px] font-medium">{fmt}</span>
            {/each}
          </div>
        </div>
        <div class="about-row">
          <span class="about-label">Built with</span>
          <span class="text-[var(--text-secondary)] text-xs">Rust · Tauri · Svelte</span>
        </div>
      </div>

      <div class="dialog-footer">
        <button
          class="btn-full bg-[var(--bg-tertiary)] hover:bg-[var(--bg-hover)] text-[var(--text-primary)] border border-[var(--border)]"
          onclick={() => store.showAbout = false}
        >
          Close
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Properties Dialog -->
{#if store.showProperties}
  {@const selected = store.selectedEntries}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
    onclick={(e) => { if (e.target === e.currentTarget) store.showProperties = false; }}
    onkeydown={(e) => { if (e.key === "Escape") store.showProperties = false; }}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div class="dialog-card w-[460px]">
      <div class="dialog-header border-b border-[var(--border)]">
        <div class="icon-badge bg-[var(--bg-tertiary)]">
          {#if selected.length === 1 && selected[0].is_directory}
            <Folder size={15} class="text-blue-400" />
          {:else}
            <File size={15} class="text-[var(--accent)]" />
          {/if}
        </div>
        <div class="flex-1 min-w-0">
          <p class="text-sm font-semibold text-[var(--text-primary)] truncate leading-tight">
            {selected.length === 1 ? selected[0].name : `${selected.length} items selected`}
          </p>
          <p class="text-[10px] text-[var(--text-muted)] leading-tight mt-0.5">Properties</p>
        </div>
        <button class="close-btn shrink-0" onclick={() => store.showProperties = false}>
          <X size={14} />
        </button>
      </div>

      <div class="dialog-body">
        {#if selected.length === 1}
          {@const e = selected[0]}
          <div class="prop-table">
            <div class="prop-row even"><span class="prop-key">Name</span><span class="prop-val truncate">{e.name}</span></div>
            <div class="prop-row odd"><span class="prop-key">Path</span><span class="prop-val truncate">{e.path}</span></div>
            <div class="prop-row even"><span class="prop-key">Type</span><span class="prop-val">{fileTypeDisplay(e.name, e.is_directory)}</span></div>
            <div class="prop-row odd"><span class="prop-key">Size</span><span class="prop-val">{formatSize(e.uncompressed_size)}</span></div>
            <div class="prop-row even"><span class="prop-key">Packed</span><span class="prop-val">{formatSize(e.compressed_size)}</span></div>
            <div class="prop-row odd"><span class="prop-key">Ratio</span><span class="prop-val">{formatRatio(e.compressed_size, e.uncompressed_size) || "—"}</span></div>
            <div class="prop-row even"><span class="prop-key">Method</span><span class="prop-val">{e.compression_method || "—"}</span></div>
            <div class="prop-row odd"><span class="prop-key">Modified</span><span class="prop-val">{e.modified || "—"}</span></div>
            {#if e.crc32 != null}
              <div class="prop-row even"><span class="prop-key">CRC-32</span><span class="prop-val font-mono">{e.crc32.toString(16).toUpperCase().padStart(8, "0")}</span></div>
            {/if}
            <div class="prop-row odd"><span class="prop-key">Encrypted</span><span class="prop-val">{e.encrypted ? "Yes" : "No"}</span></div>
          </div>
        {:else}
          {@const totalSize = selected.reduce((s, e) => s + e.uncompressed_size, 0)}
          {@const totalPacked = selected.reduce((s, e) => s + e.compressed_size, 0)}
          {@const dirs = selected.filter((e) => e.is_directory).length}
          <div class="prop-table">
            <div class="prop-row even"><span class="prop-key">Items</span><span class="prop-val">{selected.length}</span></div>
            <div class="prop-row odd"><span class="prop-key">Files</span><span class="prop-val">{selected.length - dirs}</span></div>
            <div class="prop-row even"><span class="prop-key">Folders</span><span class="prop-val">{dirs}</span></div>
            <div class="prop-row odd"><span class="prop-key">Total size</span><span class="prop-val">{formatSize(totalSize)}</span></div>
            <div class="prop-row even"><span class="prop-key">Packed</span><span class="prop-val">{formatSize(totalPacked)}</span></div>
            <div class="prop-row odd"><span class="prop-key">Ratio</span><span class="prop-val">{formatRatio(totalPacked, totalSize) || "—"}</span></div>
          </div>
        {/if}
      </div>

      <div class="dialog-footer">
        <button
          class="btn-full bg-[var(--bg-tertiary)] hover:bg-[var(--bg-hover)] text-[var(--text-primary)] border border-[var(--border)]"
          onclick={() => store.showProperties = false}
        >
          Close
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .dialog-card {
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 20px;
    box-shadow: 0 25px 60px rgba(0, 0, 0, 0.5), 0 8px 24px rgba(0, 0, 0, 0.3);
    overflow: hidden;
    animation: modalIn 0.2s cubic-bezier(0.34, 1.4, 0.64, 1);
  }

  @keyframes modalIn {
    from { opacity: 0; transform: scale(0.92) translateY(8px); }
    to   { opacity: 1; transform: scale(1) translateY(0); }
  }

  .dialog-header {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 14px 16px;
  }

  .dialog-body {
    padding: 16px 20px;
  }

  .dialog-footer {
    padding: 0 20px 20px;
  }

  .dialog-title {
    font-size: 13px;
    font-weight: 600;
    flex: 1;
  }

  .icon-badge {
    width: 32px;
    height: 32px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .close-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 26px;
    height: 26px;
    border-radius: 6px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    transition: background 0.15s, color 0.15s;
  }

  .close-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .btn-full {
    display: block;
    width: 100%;
    padding: 9px 0;
    border-radius: 10px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.15s;
    text-align: center;
  }

  .about-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }

  .about-label {
    font-size: 12px;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .prop-table {
    border: 1px solid var(--border);
    border-radius: 10px;
    overflow: hidden;
  }

  .prop-row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 7px 12px;
  }

  .prop-row.even {
    background: var(--bg-primary);
  }

  .prop-row.odd {
    background: transparent;
  }

  .prop-key {
    font-size: 11px;
    color: var(--text-muted);
    font-weight: 500;
    min-width: 68px;
    flex-shrink: 0;
  }

  .prop-val {
    font-size: 12px;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
