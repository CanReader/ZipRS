<script lang="ts">
  import { FolderOpen, FilePlus2, FileArchive, Package } from "lucide-svelte";
  import { onMount } from "svelte";

  interface Props {
    onOpen: () => void;
    onCreate: () => void;
  }

  let { onOpen, onCreate }: Props = $props();
  let visible = $state(false);

  onMount(() => {
    // Small delay so the CSS transitions actually play
    requestAnimationFrame(() => {
      visible = true;
    });
  });
</script>

<div class="welcome">
  <!-- Background decoration -->
  <div class="bg-orb orb1" class:visible></div>
  <div class="bg-orb orb2" class:visible></div>

  <div class="content">
    <!-- Logo area -->
    <div class="logo-area">
      <img
        src="/Logo.png"
        alt="ZipRS"
        class="logo"
        class:visible
      />
      <div class="title-block">
        <h1 class="app-title" class:visible>ZipRS</h1>
        <p class="app-desc" class:visible>Professional Archive Manager</p>
      </div>
    </div>

    <!-- Action cards -->
    <div class="cards">
      <button class="card card-open" class:visible onclick={onOpen}>
        <div class="card-icon blue">
          <FolderOpen size={32} strokeWidth={1.8} />
        </div>
        <div class="card-body">
          <h3>Open Archive</h3>
          <p>Browse, view, and extract files from ZIP, TAR, GZ, BZ2, ZST archives</p>
        </div>
        <kbd>Ctrl+O</kbd>
      </button>

      <button class="card card-create" class:visible onclick={onCreate}>
        <div class="card-icon green">
          <FilePlus2 size={32} strokeWidth={1.8} />
        </div>
        <div class="card-body">
          <h3>Create Archive</h3>
          <p>Compress files into a new archive with Zstd, Gzip, BZip2 or no compression</p>
        </div>
        <kbd>Ctrl+N</kbd>
      </button>
    </div>

    <!-- Drop zone -->
    <div class="dropzone" class:visible>
      <Package size={20} strokeWidth={1.5} />
      <span>Drag & drop an archive file anywhere to open it</span>
    </div>

    <!-- Supported formats -->
    <div class="formats" class:visible>
      {#each [
        { name: "ZIP", color: "#4a90e2" },
        { name: "TAR", color: "#e2a94a" },
        { name: "TAR.GZ", color: "#4abb8a" },
        { name: "TAR.BZ2", color: "#9b59b6" },
        { name: "TAR.ZST", color: "#e25555" },
      ] as fmt, i}
        <span
          class="fmt-tag"
          class:visible
          style="--tag-color: {fmt.color}; transition-delay: {0.65 + i * 0.06}s"
        >{fmt.name}</span>
      {/each}
    </div>
  </div>
</div>

<style>
  .welcome {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    background: var(--bg-deep);
    position: relative;
    overflow: hidden;
  }

  /* --- Background orbs --- */
  .bg-orb {
    position: absolute;
    border-radius: 50%;
    filter: blur(80px);
    pointer-events: none;
    opacity: 0;
    transform: scale(0.6);
    transition: opacity 1.2s ease, transform 1.2s ease;
  }
  .bg-orb.visible {
    opacity: 0.5;
    transform: scale(1);
  }
  .orb1 {
    top: -15%;
    left: 30%;
    width: 500px;
    height: 500px;
    background: radial-gradient(circle, rgba(74, 144, 226, 0.08) 0%, transparent 70%);
  }
  .orb2 {
    bottom: -20%;
    right: 25%;
    width: 400px;
    height: 400px;
    background: radial-gradient(circle, rgba(74, 187, 138, 0.05) 0%, transparent 70%);
  }

  .content {
    display: flex;
    flex-direction: column;
    align-items: center;
    z-index: 1;
    margin-top: -30px;
  }

  /* --- Logo --- */
  .logo-area {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-bottom: 40px;
  }

  .logo {
    width: 96px;
    height: 96px;
    object-fit: contain;
    filter: drop-shadow(0 4px 20px rgba(74, 144, 226, 0.25));
    margin-bottom: 16px;
    opacity: 0;
    transform: translateY(-30px) scale(0.8);
    transition: opacity 0.7s cubic-bezier(0.16, 1, 0.3, 1),
                transform 0.7s cubic-bezier(0.16, 1, 0.3, 1);
    transition-delay: 0.05s;
  }
  .logo.visible {
    opacity: 1;
    transform: translateY(0) scale(1);
  }

  .title-block {
    text-align: center;
  }

  /* --- Title --- */
  .app-title {
    font-size: 32px;
    font-weight: 700;
    margin: 0;
    background: linear-gradient(135deg, #6ab0ff 0%, #4ecdc4 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    letter-spacing: -0.3px;
    opacity: 0;
    transform: translateY(-20px);
    transition: opacity 0.6s cubic-bezier(0.16, 1, 0.3, 1),
                transform 0.6s cubic-bezier(0.16, 1, 0.3, 1);
    transition-delay: 0.15s;
  }
  .app-title.visible {
    opacity: 1;
    transform: translateY(0);
  }

  /* --- Subtitle --- */
  .app-desc {
    margin: 4px 0 0;
    font-size: 13px;
    color: var(--text-muted);
    letter-spacing: 0.3px;
    opacity: 0;
    transform: translateY(-12px);
    transition: opacity 0.5s ease, transform 0.5s ease;
    transition-delay: 0.25s;
  }
  .app-desc.visible {
    opacity: 1;
    transform: translateY(0);
  }

  /* --- Cards --- */
  .cards {
    display: flex;
    gap: 16px;
    margin-bottom: 36px;
  }

  .card {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    width: 260px;
    padding: 28px 24px 22px;
    border-radius: 16px;
    border: 1px solid var(--border);
    background: var(--bg-tertiary);
    cursor: pointer;
    text-align: left;
    position: relative;
    overflow: hidden;
    opacity: 0;
    transform: translateY(24px);
    transition: opacity 0.55s cubic-bezier(0.16, 1, 0.3, 1),
                transform 0.55s cubic-bezier(0.16, 1, 0.3, 1),
                border-color 0.2s,
                box-shadow 0.2s;
  }
  .card-open {
    transition-delay: 0.3s;
  }
  .card-create {
    transition-delay: 0.4s;
  }
  .card.visible {
    opacity: 1;
    transform: translateY(0);
  }
  .card.visible:hover {
    border-color: var(--accent);
    transform: translateY(-3px);
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.3);
  }
  .card.visible:active {
    transform: translateY(-1px);
  }

  .card::before {
    content: "";
    position: absolute;
    inset: 0;
    border-radius: 16px;
    opacity: 0;
    transition: opacity 0.25s;
  }

  .card-open::before {
    background: linear-gradient(135deg, rgba(74, 144, 226, 0.08) 0%, transparent 60%);
  }
  .card-create::before {
    background: linear-gradient(135deg, rgba(74, 187, 138, 0.08) 0%, transparent 60%);
  }

  .card:hover::before {
    opacity: 1;
  }

  .card-icon {
    width: 56px;
    height: 56px;
    border-radius: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 20px;
  }

  .card-icon.blue {
    background: rgba(74, 144, 226, 0.12);
    color: #5ba0f2;
  }
  .card-icon.green {
    background: rgba(74, 187, 138, 0.12);
    color: #4abb8a;
  }

  .card-body {
    flex: 1;
  }

  .card-body h3 {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0 0 8px;
  }

  .card-body p {
    font-size: 12px;
    color: var(--text-muted);
    line-height: 1.5;
    margin: 0;
  }

  .card kbd {
    display: inline-block;
    margin-top: 16px;
    font-size: 11px;
    padding: 4px 10px;
    border-radius: 6px;
    background: var(--bg-hover);
    color: var(--text-muted);
    border: 1px solid var(--border);
    font-family: inherit;
  }

  /* --- Drop zone --- */
  .dropzone {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 14px 28px;
    border-radius: 12px;
    border: 1.5px dashed var(--border);
    color: var(--text-muted);
    font-size: 13px;
    margin-bottom: 24px;
    opacity: 0;
    transform: translateY(16px);
    transition: opacity 0.5s ease, transform 0.5s ease;
    transition-delay: 0.5s;
  }
  .dropzone.visible {
    opacity: 1;
    transform: translateY(0);
  }

  /* --- Format tags --- */
  .formats {
    display: flex;
    gap: 8px;
  }

  .fmt-tag {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.8px;
    padding: 4px 14px;
    border-radius: 20px;
    background: color-mix(in srgb, var(--tag-color) 10%, transparent);
    color: color-mix(in srgb, var(--tag-color) 80%, white);
    opacity: 0;
    transform: translateY(10px) scale(0.9);
    transition: opacity 0.4s ease, transform 0.4s ease;
  }
  .fmt-tag.visible {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
</style>
