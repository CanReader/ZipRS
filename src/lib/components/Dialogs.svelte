<script lang="ts">
  import { store } from "$lib/store.svelte";
  import { formatSize, formatRatio } from "$lib/utils";
  import { X, AlertCircle, CheckCircle, Info } from "lucide-svelte";

  interface Props {
    onAction: (action: string) => void;
  }

  let { onAction }: Props = $props();
</script>

<!-- Error Dialog -->
{#if store.errorMessage}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
    <div class="bg-[var(--bg-secondary)] border border-[var(--border)] rounded-xl shadow-2xl p-6 w-[420px]">
      <div class="flex items-start gap-3">
        <AlertCircle size={20} class="text-red-400 shrink-0 mt-0.5" />
        <div class="flex-1">
          <h3 class="text-sm font-semibold text-red-400 mb-2">Error</h3>
          <p class="text-xs text-[var(--text-secondary)] leading-relaxed">{store.errorMessage}</p>
        </div>
        <button onclick={() => store.errorMessage = null} class="text-[var(--text-muted)] hover:text-[var(--text-primary)] bg-transparent border-none cursor-pointer">
          <X size={16} />
        </button>
      </div>
      <div class="flex justify-end mt-4">
        <button onclick={() => store.errorMessage = null}
          class="px-4 py-1.5 text-xs bg-[var(--bg-tertiary)] hover:bg-[var(--bg-hover)] border border-[var(--border)] rounded-md cursor-pointer text-[var(--text-primary)]">
          OK
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Info Dialog -->
{#if store.infoMessage}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
    <div class="bg-[var(--bg-secondary)] border border-[var(--border)] rounded-xl shadow-2xl p-6 w-[420px]">
      <div class="flex items-start gap-3">
        <CheckCircle size={20} class="text-emerald-400 shrink-0 mt-0.5" />
        <div class="flex-1">
          <h3 class="text-sm font-semibold mb-2">Success</h3>
          <p class="text-xs text-[var(--text-secondary)] leading-relaxed">{store.infoMessage}</p>
        </div>
        <button onclick={() => store.infoMessage = null} class="text-[var(--text-muted)] hover:text-[var(--text-primary)] bg-transparent border-none cursor-pointer">
          <X size={16} />
        </button>
      </div>
      <div class="flex justify-end mt-4">
        <button onclick={() => store.infoMessage = null}
          class="px-4 py-1.5 text-xs bg-[var(--accent)] hover:bg-[var(--accent-hover)] rounded-md cursor-pointer text-white border-none">
          OK
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- About Dialog -->
{#if store.showAbout}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
    <div class="bg-[var(--bg-secondary)] border border-[var(--border)] rounded-xl shadow-2xl p-8 w-[380px] text-center">
      <img src="/Logo.png" alt="ZipRS" class="w-20 h-20 mx-auto mb-3" />
      <h2 class="text-2xl font-bold bg-gradient-to-r from-blue-400 to-cyan-400 bg-clip-text text-transparent">ZipRS</h2>
      <p class="text-xs text-[var(--text-muted)] mt-1">Archive Manager</p>
      <p class="text-xs text-[var(--text-secondary)] mt-3">Version 0.1.0</p>
      <div class="mt-4 text-xs text-[var(--text-secondary)]">
        <p class="font-semibold mb-1">Supported formats:</p>
        <p>ZIP (Zstd), TAR, TAR.GZ, TAR.BZ2, TAR.ZST</p>
      </div>
      <p class="text-[10px] text-[var(--text-muted)] mt-4">Built with Rust + Tauri + Svelte</p>
      <button onclick={() => store.showAbout = false}
        class="mt-5 px-6 py-1.5 text-xs bg-[var(--bg-tertiary)] hover:bg-[var(--bg-hover)] border border-[var(--border)] rounded-md cursor-pointer text-[var(--text-primary)]">
        Close
      </button>
    </div>
  </div>
{/if}

<!-- Properties Dialog -->
{#if store.showProperties}
  {@const selected = store.selectedEntries}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
    <div class="bg-[var(--bg-secondary)] border border-[var(--border)] rounded-xl shadow-2xl p-6 w-[420px]">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-sm font-semibold">
          {selected.length === 1 ? `Properties — ${selected[0].name}` : `Properties — ${selected.length} items`}
        </h3>
        <button onclick={() => store.showProperties = false} class="text-[var(--text-muted)] hover:text-[var(--text-primary)] bg-transparent border-none cursor-pointer">
          <X size={16} />
        </button>
      </div>

      {#if selected.length === 1}
        {@const e = selected[0]}
        <div class="space-y-1.5 text-xs">
          <div class="prop-row"><span class="prop-label">Name:</span><span>{e.name}</span></div>
          <div class="prop-row"><span class="prop-label">Path:</span><span class="truncate">{e.path}</span></div>
          <div class="prop-row"><span class="prop-label">Type:</span><span>{e.is_directory ? "Folder" : e.name.split(".").pop()?.toUpperCase() + " File"}</span></div>
          <div class="prop-row"><span class="prop-label">Size:</span><span>{formatSize(e.uncompressed_size)}</span></div>
          <div class="prop-row"><span class="prop-label">Packed:</span><span>{formatSize(e.compressed_size)}</span></div>
          <div class="prop-row"><span class="prop-label">Ratio:</span><span>{formatRatio(e.compressed_size, e.uncompressed_size)}</span></div>
          <div class="prop-row"><span class="prop-label">Method:</span><span>{e.compression_method}</span></div>
          <div class="prop-row"><span class="prop-label">Modified:</span><span>{e.modified || "—"}</span></div>
          {#if e.crc32 != null}
            <div class="prop-row"><span class="prop-label">CRC-32:</span><span>{e.crc32.toString(16).toUpperCase().padStart(8, "0")}</span></div>
          {/if}
          <div class="prop-row"><span class="prop-label">Encrypted:</span><span>{e.encrypted ? "Yes" : "No"}</span></div>
        </div>
      {:else}
        {@const totalSize = selected.reduce((s, e) => s + e.uncompressed_size, 0)}
        {@const totalPacked = selected.reduce((s, e) => s + e.compressed_size, 0)}
        {@const dirs = selected.filter((e) => e.is_directory).length}
        <div class="space-y-1.5 text-xs">
          <div class="prop-row"><span class="prop-label">Items:</span><span>{selected.length}</span></div>
          <div class="prop-row"><span class="prop-label">Files:</span><span>{selected.length - dirs}</span></div>
          <div class="prop-row"><span class="prop-label">Folders:</span><span>{dirs}</span></div>
          <div class="prop-row"><span class="prop-label">Total size:</span><span>{formatSize(totalSize)}</span></div>
          <div class="prop-row"><span class="prop-label">Total packed:</span><span>{formatSize(totalPacked)}</span></div>
          <div class="prop-row"><span class="prop-label">Overall ratio:</span><span>{formatRatio(totalPacked, totalSize)}</span></div>
        </div>
      {/if}

      <div class="flex justify-end mt-5">
        <button onclick={() => store.showProperties = false}
          class="px-4 py-1.5 text-xs bg-[var(--bg-tertiary)] hover:bg-[var(--bg-hover)] border border-[var(--border)] rounded-md cursor-pointer text-[var(--text-primary)]">
          Close
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .prop-row {
    display: flex;
    gap: 8px;
    padding: 3px 0;
  }
  .prop-label {
    font-weight: 600;
    color: var(--text-secondary);
    min-width: 80px;
  }
</style>
