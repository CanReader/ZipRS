<script lang="ts">
  import { store } from "$lib/store.svelte";

  let percent = $derived(
    store.progress.total > 0
      ? Math.round((store.progress.current / store.progress.total) * 100)
      : 0
  );
</script>

{#if store.isBusy}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
    <div class="bg-[var(--bg-secondary)] border border-[var(--border)] rounded-xl shadow-2xl p-6 w-[420px]">
      <h3 class="text-sm font-semibold mb-3">Operation in Progress</h3>
      <p class="text-xs text-[var(--text-secondary)] mb-3 truncate">{store.progress.message}</p>

      <!-- Progress bar -->
      <div class="h-2 bg-[var(--bg-primary)] rounded-full overflow-hidden mb-2">
        <div
          class="h-full bg-gradient-to-r from-blue-500 to-cyan-400 rounded-full transition-all duration-300"
          style="width: {percent}%"
        ></div>
      </div>

      <div class="flex justify-between text-[11px] text-[var(--text-muted)]">
        <span>{store.progress.current} / {store.progress.total}</span>
        <span>{percent}%</span>
      </div>
    </div>
  </div>
{/if}
