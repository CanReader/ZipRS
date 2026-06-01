<script lang="ts">
  import { Package, Loader2 } from "lucide-svelte";
  import { store } from "$lib/store.svelte";

  let percent = $derived(
    store.progress.total > 0
      ? Math.round((store.progress.current / store.progress.total) * 100)
      : 0
  );

  let filename = $derived.by(() => {
    const msg = store.progress.message;
    const colonIdx = msg.indexOf(": ");
    return colonIdx >= 0 ? msg.substring(colonIdx + 2) : "";
  });

  let label = $derived.by(() => {
    const msg = store.progress.message;
    const colonIdx = msg.indexOf(": ");
    return colonIdx >= 0 ? msg.substring(0, colonIdx) : msg;
  });

  let isIndeterminate = $derived(store.progress.total === 0);
</script>

{#if store.isBusy}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
    style="animation: fadeIn 0.15s ease-out"
  >
    <div
      class="bg-[var(--bg-secondary)] border border-[var(--border)] rounded-2xl shadow-2xl p-7 w-[460px] flex flex-col gap-5"
      style="animation: popIn 0.18s cubic-bezier(0.34,1.56,0.64,1)"
    >
      <!-- Header -->
      <div class="flex items-center gap-3">
        <div class="w-9 h-9 rounded-xl bg-[var(--accent)]/15 flex items-center justify-center shrink-0">
          <Package size={18} class="text-[var(--accent)]" />
        </div>
        <div class="flex-1 min-w-0">
          <p class="text-sm font-semibold text-[var(--text-primary)] leading-tight">
            {label}
          </p>
          {#if filename}
            <p class="text-[11px] text-[var(--text-muted)] mt-0.5 truncate font-mono">{filename}</p>
          {/if}
        </div>
        <Loader2 size={16} class="text-[var(--accent)] shrink-0 animate-spin" />
      </div>

      <!-- Progress bar -->
      <div class="flex flex-col gap-2">
        <div class="h-1.5 bg-[var(--bg-primary)] rounded-full overflow-hidden">
          {#if isIndeterminate}
            <div class="h-full w-1/3 bg-gradient-to-r from-[var(--accent)] to-cyan-400 rounded-full indeterminate-bar"></div>
          {:else}
            <div
              class="h-full bg-gradient-to-r from-[var(--accent)] to-cyan-400 rounded-full transition-all duration-200"
              style="width: {percent}%"
            ></div>
          {/if}
        </div>

        <!-- Footer -->
        {#if !isIndeterminate}
          <div class="flex items-center justify-between">
            <span class="text-[11px] text-[var(--text-muted)]">
              {store.progress.current} of {store.progress.total}
            </span>
            <span class="text-[11px] font-semibold text-[var(--accent)]">{percent}%</span>
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  @keyframes fadeIn {
    from { opacity: 0; }
    to   { opacity: 1; }
  }
  @keyframes popIn {
    from { opacity: 0; transform: scale(0.92) translateY(8px); }
    to   { opacity: 1; transform: scale(1) translateY(0); }
  }
  @keyframes indeterminate {
    0%   { transform: translateX(-100%); }
    100% { transform: translateX(400%); }
  }
  .indeterminate-bar {
    animation: indeterminate 1.2s ease-in-out infinite;
  }
</style>
