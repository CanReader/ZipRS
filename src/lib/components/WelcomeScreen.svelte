<script lang="ts">
  import { FolderOpen, FilePlus2, FileArchive, Package } from "lucide-svelte";

  interface Props {
    onOpen: () => void;
    onCreate: () => void;
  }

  let { onOpen, onCreate }: Props = $props();
</script>

<div class="welcome">
  <!-- Background decoration -->
  <div class="bg-orb orb1"></div>
  <div class="bg-orb orb2"></div>

  <div class="content">
    <!-- Logo area -->
    <div class="logo-area">
      <img src="/Logo.png" alt="ZipRS" class="logo" />
      <div class="title-block">
        <h1 class="app-title">ZipRS</h1>
        <p class="app-desc">Professional Archive Manager</p>
      </div>
    </div>

    <!-- Action cards -->
    <div class="cards">
      <button class="card card-open" onclick={onOpen}>
        <div class="card-icon blue">
          <FolderOpen size={32} strokeWidth={1.8} />
        </div>
        <div class="card-body">
          <h3>Open Archive</h3>
          <p>Browse, view, and extract files from ZIP, TAR, GZ, BZ2, ZST archives</p>
        </div>
        <kbd>Ctrl+O</kbd>
      </button>

      <button class="card card-create" onclick={onCreate}>
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
    <div class="dropzone">
      <Package size={20} strokeWidth={1.5} />
      <span>Drag & drop an archive file anywhere to open it</span>
    </div>

    <!-- Supported formats -->
    <div class="formats">
      {#each [
        { name: "ZIP", color: "#4a90e2" },
        { name: "TAR", color: "#e2a94a" },
        { name: "TAR.GZ", color: "#4abb8a" },
        { name: "TAR.BZ2", color: "#9b59b6" },
        { name: "TAR.ZST", color: "#e25555" },
      ] as fmt}
        <span class="fmt-tag" style="--tag-color: {fmt.color}">{fmt.name}</span>
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

  .bg-orb {
    position: absolute;
    border-radius: 50%;
    filter: blur(80px);
    pointer-events: none;
    opacity: 0.5;
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

  /* Logo */
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
  }

  .title-block {
    text-align: center;
  }

  .app-title {
    font-size: 32px;
    font-weight: 700;
    margin: 0;
    background: linear-gradient(135deg, #6ab0ff 0%, #4ecdc4 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    letter-spacing: -0.3px;
  }

  .app-desc {
    margin: 4px 0 0;
    font-size: 13px;
    color: var(--text-muted);
    letter-spacing: 0.3px;
  }

  /* Cards */
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
    transition: all 0.25s ease;
    text-align: left;
    position: relative;
    overflow: hidden;
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

  .card:hover {
    border-color: var(--accent);
    transform: translateY(-3px);
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.3);
  }

  .card:hover::before {
    opacity: 1;
  }

  .card:active {
    transform: translateY(-1px);
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

  /* Drop zone */
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
  }

  /* Format tags */
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
  }
</style>
