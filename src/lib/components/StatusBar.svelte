<script lang="ts">
  import { store } from "$lib/store.svelte";
  import { formatSize } from "$lib/utils";

  let totalSize = $derived(
    store.visibleEntries.reduce((sum, e) => sum + e.uncompressed_size, 0)
  );
  let entryCount = $derived(store.visibleEntries.length);
  let selCount = $derived(store.selectedIndices.length);
  let selSize = $derived(
    store.selectedEntries.reduce((sum, e) => sum + e.uncompressed_size, 0)
  );
</script>

<div class="flex items-center h-6 px-3 bg-[var(--bg-secondary)] border-t border-[var(--border)] text-[11px] text-[var(--text-muted)]">
  <div class="flex-1 flex items-center gap-4">
    {#if store.hasArchive}
      <span>{entryCount} item{entryCount !== 1 ? "s" : ""}</span>
      <span>{formatSize(totalSize)}</span>
      {#if selCount > 0}
        <span class="text-[var(--accent)]">{selCount} selected ({formatSize(selSize)})</span>
      {/if}
    {/if}
  </div>
  <div class="text-right truncate max-w-[50%]">
    {store.statusMessage}
  </div>
</div>
