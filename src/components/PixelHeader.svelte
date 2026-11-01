<script lang="ts">
  import { getTheme, toggleTheme as toggle } from "../stores/theme.svelte";
  import { t } from "../stores/i18n.svelte";

  type NavPage = "home" | "history" | "settings";
  let { title, showBack = false, showThemeToggle = true, showSettings = true, onnavigate }: {
    title: string;
    showBack?: boolean;
    showThemeToggle?: boolean;
    showSettings?: boolean;
    onnavigate?: (page: NavPage) => void;
  } = $props();
</script>

<header class="pixel-header">
  <div class="header-left">
    {#if showBack}
      <button class="back-btn" onclick={() => onnavigate?.("home")}>
        ← {t("nav.home")}
      </button>
    {/if}
    <h1 class="title">
      <span class="icon">■</span>
      {title}
    </h1>
  </div>

  <div class="header-right">
    {#if showThemeToggle}
      <button class="icon-btn" onclick={toggle} title={t("settings.theme")} aria-label={t("settings.theme")}>
        {#if getTheme() === "dark"}
          ☀
        {:else}
          🌙
        {/if}
      </button>
    {/if}

    {#if showSettings}
      <button class="icon-btn" onclick={() => onnavigate?.("settings")} title={t("nav.settings")} aria-label={t("nav.settings")}>
        ⚙
      </button>
    {/if}
  </div>
</header>

<style>
  .pixel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 0;
    margin-bottom: 20px;
    border-bottom: 2px solid var(--pixel-border);
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .title {
    font-family: var(--font-pixel);
    font-size: 16px;
    color: var(--accent-2);
    text-shadow: 2px 2px 0 rgba(0,0,0,0.5);
  }

  .icon {
    color: var(--accent-3);
  }

  .back-btn {
    font-family: var(--font-pixel);
    font-size: 10px;
    background: var(--accent-1);
    color: var(--text-primary);
    border: 2px solid var(--pixel-border);
    padding: 6px 12px;
    cursor: pointer;
    box-shadow: var(--shadow-pixel-3);
  }

  .back-btn:hover {
    background: var(--accent-2);
    transform: translate(1px, 1px);
    box-shadow: var(--shadow-pixel-2);
  }

  .header-right {
    display: flex;
    gap: 8px;
  }

  .icon-btn {
    font-family: var(--font-pixel);
    font-size: 16px;
    background: var(--bg-secondary);
    color: var(--text-primary);
    border: 2px solid var(--pixel-border);
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    box-shadow: var(--shadow-pixel-3);
  }

  .icon-btn:hover {
    background: var(--accent-1);
    transform: translate(1px, 1px);
    box-shadow: var(--shadow-pixel-2);
  }
</style>
