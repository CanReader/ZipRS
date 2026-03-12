<script lang="ts">
  import { ChevronRight, Search, Archive } from "lucide-svelte";
  import { store } from "$lib/store.svelte";

  let searchInput = $state("");

  function onSearch() {
    store.searchQuery = searchInput;
  }

  function navigateToSegment(index: number) {
    if (index < 0) {
      store.navigateTo("");
    } else {
      const segments = store.currentPath.split("/");
      const path = segments.slice(0, index + 1).join("/");
      store.navigateTo(path);
    }
  }

  let segments = $derived(store.currentPath ? store.currentPath.split("/") : []);
  let archiveName = $derived(
    store.archivePath ? store.archivePath.split("/").pop() || store.archivePath : ""
  );
</script>

<div class="flex items-center h-8 px-3 gap-2 bg-[var(--bg-primary)] border-b border-[var(--border)]">
  <!-- Breadcrumbs -->
  <div class="flex items-center gap-0.5 flex-1 min-w-0 overflow-hidden">
    {#if store.archiveFormat}
      <span class="text-[10px] font-bold px-1.5 py-0.5 rounded bg-[var(--accent)]/20 text-[var(--accent)] shrink-0">
        {store.archiveFormat}
      </span>
      <ChevronRight size={12} class="text-[var(--text-muted)] shrink-0" />
    {/if}

    <button onclick={() => navigateToSegment(-1)}
      class="text-xs hover:text-[var(--accent)] transition-colors truncate shrink-0 bg-transparent border-none cursor-pointer px-1 py-0.5 rounded hover:bg-[var(--bg-tertiary)]"
      style="color: var(--text-secondary)">
      {archiveName}
    </button>

    {#each segments as segment, i}
      <ChevronRight size={12} class="text-[var(--text-muted)] shrink-0" />
      <button onclick={() => navigateToSegment(i)}
        class="text-xs hover:text-[var(--accent)] transition-colors truncate shrink-0 bg-transparent border-none cursor-pointer px-1 py-0.5 rounded hover:bg-[var(--bg-tertiary)]"
        style="color: {i === segments.length - 1 ? 'var(--text-primary)' : 'var(--text-secondary)'}">
        {segment}
      </button>
    {/each}
  </div>

  <!-- Search -->
  <div class="flex items-center gap-1.5 bg-[var(--bg-tertiary)] rounded-md px-2 py-1 border border-transparent focus-within:border-[var(--accent)]/50 transition-colors">
    <Search size={12} class="text-[var(--text-muted)]" />
    <input
      type="text"
      placeholder="Search..."
      bind:value={searchInput}
      oninput={onSearch}
      class="bg-transparent border-none outline-none text-xs w-32 text-[var(--text-primary)] placeholder:text-[var(--text-muted)]"
    />
  </div>
</div>
