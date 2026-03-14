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
    requestAnimationFrame(() => {
      visible = true;
    });
  });
</script>

<div class="welcome">
  <!-- Background orbs -->
  <div class="bg-orb orb1" class:visible></div>
  <div class="bg-orb orb2" class:visible></div>
  <div class="bg-orb orb3" class:visible></div>

  <div class="content">
    <!-- Logo -->
    <div class="logo-area">
      <div class="logo-ring" class:visible>
        <img src="/Logo.png" alt="ZipRS" class="logo" class:visible />
      </div>
      <div class="title-block">
        <h1 class="app-title" class:visible>
          <span class="title-letter" style="transition-delay: 0.12s">Z</span><span class="title-letter" style="transition-delay: 0.15s">i</span><span class="title-letter" style="transition-delay: 0.18s">p</span><span class="title-letter" style="transition-delay: 0.21s">R</span><span class="title-letter" style="transition-delay: 0.24s">S</span>
        </h1>
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
      <div class="dropzone-icon">
        <Package size={20} strokeWidth={1.5} />
      </div>
      <span>Drag & drop an archive file anywhere to open it</span>
    </div>

    <!-- Format tags -->
    <div class="formats">
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
          style="--tag-color: {fmt.color}; transition-delay: {0.38 + i * 0.05}s"
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

  /* --- Background orbs with float animation --- */
  .bg-orb {
    position: absolute;
    border-radius: 50%;
    filter: blur(80px);
    pointer-events: none;
    opacity: 0;
    transform: scale(0.5);
    transition: opacity 1s ease, transform 1s ease;
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
    animation: float1 8s ease-in-out infinite;
    animation-delay: 1s;
  }
  .orb2 {
    bottom: -20%;
    right: 25%;
    width: 400px;
    height: 400px;
    background: radial-gradient(circle, rgba(74, 187, 138, 0.06) 0%, transparent 70%);
    animation: float2 10s ease-in-out infinite;
    animation-delay: 2s;
  }
  .orb3 {
    top: 40%;
    left: -10%;
    width: 350px;
    height: 350px;
    background: radial-gradient(circle, rgba(180, 100, 255, 0.04) 0%, transparent 70%);
    animation: float3 12s ease-in-out infinite;
    animation-delay: 0.5s;
  }

  @keyframes float1 {
    0%, 100% { transform: translate(0, 0) scale(1); }
    50% { transform: translate(30px, -20px) scale(1.05); }
  }
  @keyframes float2 {
    0%, 100% { transform: translate(0, 0) scale(1); }
    50% { transform: translate(-25px, 15px) scale(1.08); }
  }
  @keyframes float3 {
    0%, 100% { transform: translate(0, 0) scale(1); }
    50% { transform: translate(20px, 25px) scale(1.04); }
  }

  .content {
    display: flex;
    flex-direction: column;
    align-items: center;
    z-index: 1;
    margin-top: -30px;
  }

  /* --- Logo with ring pulse --- */
  .logo-area {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-bottom: 36px;
  }

  .logo-ring {
    position: relative;
    margin-bottom: 16px;
  }
  .logo-ring::after {
    content: "";
    position: absolute;
    inset: -8px;
    border-radius: 50%;
    border: 2px solid var(--accent);
    opacity: 0;
    transform: scale(0.8);
    transition: opacity 0.6s ease 0.3s, transform 0.6s ease 0.3s;
    animation: ring-breathe 3s ease-in-out infinite;
    animation-delay: 1.5s;
  }
  .logo-ring.visible::after {
    opacity: 0.15;
    transform: scale(1);
  }

  @keyframes ring-breathe {
    0%, 100% { transform: scale(1); opacity: 0.15; }
    50% { transform: scale(1.08); opacity: 0.25; }
  }

  .logo {
    width: 96px;
    height: 96px;
    object-fit: contain;
    filter: drop-shadow(0 4px 20px rgba(74, 144, 226, 0.25));
    opacity: 0;
    transform: translateY(-24px) scale(0.7) rotate(-8deg);
    transition: opacity 0.5s cubic-bezier(0.16, 1, 0.3, 1),
                transform 0.5s cubic-bezier(0.16, 1, 0.3, 1),
                filter 0.3s ease;
    transition-delay: 0.02s;
  }
  .logo.visible {
    opacity: 1;
    transform: translateY(0) scale(1) rotate(0deg);
    animation: logo-float 4s ease-in-out infinite;
    animation-delay: 0.8s;
  }
  .logo:hover {
    filter: drop-shadow(0 6px 28px rgba(74, 144, 226, 0.45));
    animation-play-state: paused;
    transform: scale(1.08);
  }

  @keyframes logo-float {
    0%, 100% { transform: translateY(0) scale(1); }
    50% { transform: translateY(-6px) scale(1.02); }
  }

  .title-block {
    text-align: center;
  }

  /* --- Title with per-letter animation --- */
  .app-title {
    font-size: 32px;
    font-weight: 700;
    margin: 0;
    background: linear-gradient(135deg, #6ab0ff 0%, #4ecdc4 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    background-size: 200% 200%;
    letter-spacing: -0.3px;
    animation: shimmer 4s ease-in-out infinite;
    animation-delay: 1s;
  }

  .title-letter {
    display: inline-block;
    opacity: 0;
    transform: translateY(-16px);
    transition: opacity 0.4s cubic-bezier(0.16, 1, 0.3, 1),
                transform 0.4s cubic-bezier(0.16, 1, 0.3, 1);
  }
  .app-title.visible .title-letter {
    opacity: 1;
    transform: translateY(0);
  }

  @keyframes shimmer {
    0%, 100% { background-position: 0% 50%; }
    50% { background-position: 100% 50%; }
  }

  /* --- Subtitle --- */
  .app-desc {
    margin: 4px 0 0;
    font-size: 13px;
    color: var(--text-muted);
    letter-spacing: 0.3px;
    opacity: 0;
    transform: translateY(-10px);
    transition: opacity 0.4s ease, transform 0.4s ease;
    transition-delay: 0.2s;
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
    transform: translateY(20px);
    transition: opacity 0.45s cubic-bezier(0.16, 1, 0.3, 1),
                transform 0.2s cubic-bezier(0.16, 1, 0.3, 1),
                border-color 0.15s,
                box-shadow 0.15s;
  }
  .card-open { transition-delay: 0.22s; }
  .card-create { transition-delay: 0.28s; }

  .card.visible {
    opacity: 1;
    transform: translateY(0);
  }
  .card.visible:hover {
    border-color: var(--accent);
    transform: translateY(-4px);
    box-shadow: 0 14px 44px rgba(0, 0, 0, 0.35);
  }
  .card.visible:hover .card-icon {
    transform: scale(1.1);
  }
  .card.visible:active {
    transform: translateY(-1px);
    box-shadow: 0 6px 20px rgba(0, 0, 0, 0.25);
  }

  .card::before {
    content: "";
    position: absolute;
    inset: 0;
    border-radius: 16px;
    opacity: 0;
    transition: opacity 0.2s;
  }
  .card-open::before {
    background: linear-gradient(135deg, rgba(74, 144, 226, 0.1) 0%, transparent 60%);
  }
  .card-create::before {
    background: linear-gradient(135deg, rgba(74, 187, 138, 0.1) 0%, transparent 60%);
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
    transition: transform 0.2s cubic-bezier(0.16, 1, 0.3, 1);
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
    transition: background 0.15s, color 0.15s;
  }
  .card:hover kbd {
    background: var(--accent);
    color: white;
    border-color: var(--accent);
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
    transform: translateY(14px);
    transition: opacity 0.4s ease, transform 0.4s ease, border-color 0.3s;
    transition-delay: 0.32s;
  }
  .dropzone.visible {
    opacity: 1;
    transform: translateY(0);
    animation: dropzone-pulse 3s ease-in-out infinite;
    animation-delay: 2s;
  }
  .dropzone:hover {
    border-color: var(--accent);
  }

  .dropzone-icon {
    animation: bounce-gentle 2s ease-in-out infinite;
    animation-delay: 1.5s;
  }

  @keyframes dropzone-pulse {
    0%, 100% { border-color: var(--border); }
    50% { border-color: var(--separator); }
  }
  @keyframes bounce-gentle {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-3px); }
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
    transform: translateY(8px) scale(0.85);
    transition: opacity 0.35s ease, transform 0.35s ease, background 0.2s, box-shadow 0.2s;
    cursor: default;
  }
  .fmt-tag.visible {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
  .fmt-tag:hover {
    background: color-mix(in srgb, var(--tag-color) 20%, transparent);
    box-shadow: 0 0 12px color-mix(in srgb, var(--tag-color) 25%, transparent);
    transform: translateY(-2px) scale(1.05);
  }
</style>
