<script lang="ts">
  import { Palette, Check } from "lucide-svelte";
  import { store } from "$lib/store.svelte";
  import { themes, applyTheme } from "$lib/themes";

  let open = $state(false);

  function selectTheme(id: string) {
    store.setTheme(id);
    open = false;
  }

  function handleClickOutside() {
    if (open) open = false;
  }
</script>

<svelte:window onclick={handleClickOutside} />

<div class="relative">
  <button
    class="tbtn"
    title="Change theme"
    onclick={(e) => { e.stopPropagation(); open = !open; }}
  >
    <Palette size={15} strokeWidth={2} />
  </button>

  {#if open}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="panel" onclick={(e) => e.stopPropagation()}>
      <div class="panel-header">Theme</div>
      <div class="theme-list">
        {#each themes as theme}
          {@const active = store.theme === theme.id}
          <button
            class="theme-item"
            class:active
            onclick={() => selectTheme(theme.id)}
          >
            <div class="preview">
              <div class="swatch" style="background: {theme.colors['--bg-primary']}"></div>
              <div class="swatch" style="background: {theme.colors['--bg-secondary']}"></div>
              <div class="swatch" style="background: {theme.colors['--accent']}"></div>
              <div class="swatch" style="background: {theme.colors['--text-primary']}"></div>
            </div>
            <span class="theme-name">{theme.name}</span>
            {#if active}
              <Check size={14} class="check" />
            {/if}
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
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
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.15s;
  }
  .tbtn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .panel {
    position: absolute;
    top: 40px;
    right: 0;
    width: 240px;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 12px;
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.4);
    z-index: 100;
    overflow: hidden;
  }

  .panel-header {
    padding: 10px 14px 6px;
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.8px;
    color: var(--text-muted);
  }

  .theme-list {
    padding: 4px 6px 8px;
    max-height: 360px;
    overflow-y: auto;
  }

  .theme-item {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    padding: 8px 10px;
    border: 1px solid transparent;
    border-radius: 8px;
    background: transparent;
    color: var(--text-primary);
    cursor: pointer;
    transition: all 0.12s;
    text-align: left;
    font-size: 12px;
  }

  .theme-item:hover {
    background: var(--bg-hover);
  }

  .theme-item.active {
    background: var(--bg-tertiary);
    border-color: var(--accent);
  }

  .preview {
    display: flex;
    border-radius: 6px;
    overflow: hidden;
    flex-shrink: 0;
    border: 1px solid rgba(128, 128, 128, 0.15);
  }

  .swatch {
    width: 14px;
    height: 22px;
  }

  .theme-name {
    flex: 1;
    font-weight: 500;
  }

  .theme-item :global(.check) {
    color: var(--accent);
    flex-shrink: 0;
  }
</style>
