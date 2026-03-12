<script lang="ts">
  import { FolderOpen, Download, FolderDown, Plus, Trash2, Info, Archive, GripVertical } from "lucide-svelte";
  import { store } from "$lib/store.svelte";

  interface Props {
    x: number;
    y: number;
    visible: boolean;
    onAction: (action: string) => void;
    onClose: () => void;
  }

  let { x, y, visible, onAction, onClose }: Props = $props();

  function handleAction(action: string) {
    onAction(action);
    onClose();
  }

  function handleClick(e: MouseEvent) {
    if (visible) onClose();
  }
</script>

<svelte:window onclick={handleClick} />

{#if visible}
  <div
    class="fixed z-50 bg-[var(--bg-secondary)] border border-[var(--border)] rounded-lg shadow-xl shadow-black/40 py-1.5 min-w-[200px]"
    style="left: {x}px; top: {y}px"
    role="menu"
  >
    <button class="ctx-item" onclick={() => handleAction("open-file")}>
      <FolderOpen size={14} class="text-blue-400" /> Open
    </button>
    <div class="h-px bg-[var(--border)] my-1 mx-2"></div>
    <button class="ctx-item" onclick={() => handleAction("extract-selected")}>
      <Download size={14} class="text-cyan-400" /> Extract selected...
    </button>
    <button class="ctx-item" onclick={() => handleAction("extract")}>
      <Archive size={14} class="text-cyan-400" /> Extract all...
    </button>
    <button class="ctx-item" onclick={() => handleAction("extract-here")}>
      <FolderDown size={14} class="text-cyan-400" /> Extract here
    </button>
    <button class="ctx-item" onclick={() => handleAction("drag-out")}>
      <GripVertical size={14} class="text-amber-400" /> Drag out to folder...
    </button>
    <div class="h-px bg-[var(--border)] my-1 mx-2"></div>
    <button class="ctx-item" onclick={() => handleAction("properties")}>
      <Info size={14} class="text-purple-400" /> Properties
    </button>
    {#if store.canModify}
      <div class="h-px bg-[var(--border)] my-1 mx-2"></div>
      <button class="ctx-item" onclick={() => handleAction("add")}>
        <Plus size={14} class="text-green-400" /> Add files...
      </button>
      <button class="ctx-item text-red-400" onclick={() => handleAction("delete")}>
        <Trash2 size={14} /> Delete
      </button>
    {/if}
  </div>
{/if}

<style>
  .ctx-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 6px 14px;
    font-size: 12px;
    background: none;
    border: none;
    color: var(--text-primary);
    cursor: pointer;
    transition: background 0.1s;
    text-align: left;
  }
  .ctx-item:hover {
    background: var(--bg-active);
    color: white;
  }
</style>
