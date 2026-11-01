<script lang="ts">
  import { onMount } from "svelte";
  import { listen, emit } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { register, unregisterAll } from "@tauri-apps/plugin-global-shortcut";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { getTheme, checkAutoTheme } from "./stores/theme.svelte";
  import { t } from "./stores/i18n.svelte";
  import { formatHMS } from "./utils/format";
  import type { Settings as SettingsType } from "./stores/settings.svelte";
  import type { TodayStats } from "./stores/stats.svelte";
  import Home from "./pages/Home.svelte";
  import History from "./pages/History.svelte";
  import Settings from "./pages/Settings.svelte";
  import AppDetail from "./pages/AppDetail.svelte";
  import ExitReminder from "./components/ExitReminder.svelte";
  import type { NavPage } from "./types";

  let currentPage: NavPage = $state("home");
  let selectedAppName = $state("");
  let showExitReminder = $state(false);
  let totalTime = $state("00:00:00");
  let topApp = $state("");
  let topAppTime = $state("");
  let appCount = $state(0);
  let isPaused = $state(false);
  let isAlwaysOnTop = $state(false);

  // Apply theme reactively
  $effect(() => {
    document.documentElement.setAttribute("data-theme", getTheme());
  });

  async function showExitSummary() {
    try {
      const s = await invoke<SettingsType>("get_settings");
      if (s.reminder_on_exit) {
        // Show window first so user can see the reminder
        try {
          const { getCurrentWindow } = await import("@tauri-apps/api/window");
          const win = getCurrentWindow();
          await win.show();
          await win.setFocus();
        } catch {
          // Window API unavailable
        }

        const stats = await invoke<TodayStats>("get_today_stats");
        totalTime = formatHMS(stats.total_sec);
        appCount = stats.app_count;

        if (stats.app_ranking.length > 0) {
          topApp = stats.app_ranking[0].process_name;
          topAppTime = formatHMS(stats.app_ranking[0].duration_sec);
        }

        showExitReminder = true;
      } else {
        quitApp();
      }
    } catch (e) {
      console.error("Failed to get stats:", e);
      quitApp();
    }
  }

  function handleReminderClose() {
    showExitReminder = false;
  }

  function handleQuit() {
    showExitReminder = false;
    quitApp();
  }

  function quitApp() {
    invoke("quit_app").catch(console.error);
  }

  function navigateTo(page: NavPage, appName?: string) {
    currentPage = page;
    if (appName) selectedAppName = appName;
  }

  async function togglePause() {
    try {
      isPaused = await invoke<boolean>("toggle_pause");
      emit("pause-changed", String(isPaused)).catch(console.error);
    } catch (e) {
      console.error("Failed to toggle pause:", e);
    }
  }

  async function toggleAlwaysOnTop() {
    try {
      isAlwaysOnTop = await invoke<boolean>("toggle_always_on_top");
    } catch (e) {
      console.error("Failed to toggle always on top:", e);
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    // Don't handle shortcuts when typing in inputs
    const tag = (e.target as HTMLElement)?.tagName;
    if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return;

    if (showExitReminder) {
      if (e.key === "Escape") handleReminderClose();
      return;
    }

    switch (e.key) {
      case "1":
        navigateTo("home");
        break;
      case "2":
        navigateTo("history");
        break;
      case "3":
        navigateTo("settings");
        break;
      case "Escape":
        if (currentPage !== "home") navigateTo("home");
        break;
      case " ":
        e.preventDefault();
        togglePause();
        break;
      case "t":
      case "T":
        toggleAlwaysOnTop();
        break;
    }
  }

  onMount(() => {
    window.addEventListener("keydown", handleKeydown);

    // Register global shortcuts
    registerGlobalShortcuts();

    // Check auto theme every 5 minutes
    const autoThemeInterval = setInterval(checkAutoTheme, 300000);

    // Listen for tray menu events
    const validPages = new Set(["home", "history", "settings"]);
    const unlistenNavigate = listen<string>("navigate", (event) => {
      const page = event.payload;
      navigateTo(validPages.has(page) ? (page as NavPage) : "home");
    });

    const unlistenQuit = listen("tray-quit", () => {
      showExitSummary();
    });

    const unlistenPause = listen("tray-pause", () => {
      togglePause();
    });

    return () => {
      window.removeEventListener("keydown", handleKeydown);
      clearInterval(autoThemeInterval);
      unlistenNavigate.then((fn) => fn());
      unlistenQuit.then((fn) => fn());
      unlistenPause.then((fn) => fn());
      unregisterAll().catch(console.error);
    };
  });

  // Register global shortcuts
  async function registerGlobalShortcuts() {
    try {
      // Ctrl+Shift+S: Show/Hide window
      await register("CommandOrControl+Shift+S", async () => {
        const win = getCurrentWindow();
        const isVisible = await win.isVisible();
        if (isVisible) {
          await win.hide();
        } else {
          await win.show();
          await win.setFocus();
        }
      });

      // Ctrl+Shift+P: Toggle pause
      await register("CommandOrControl+Shift+P", () => {
        togglePause();
      });

      // Ctrl+Shift+T: Toggle always on top
      await register("CommandOrControl+Shift+T", () => {
        toggleAlwaysOnTop();
      });
    } catch (e) {
      console.error("Failed to register global shortcuts:", e);
    }
  }
</script>

<main class="app-container">
  {#if isPaused}
    <div class="paused-banner">
      <span class="paused-icon">⏸</span>
      <span>{t("home.paused")}</span>
      <button class="resume-btn" onclick={togglePause}>{t("home.resume")}</button>
    </div>
  {/if}

  {#if isAlwaysOnTop}
    <div class="top-banner">
      <span class="top-icon">📌</span>
      <span>{t("home.pinned")}</span>
      <button class="resume-btn" onclick={toggleAlwaysOnTop}>{t("home.unpin")}</button>
    </div>
  {/if}

  {#key currentPage}
    <div class="page-enter">
      {#if currentPage === "home"}
        <Home onnavigate={navigateTo} />
      {:else if currentPage === "history"}
        <History onnavigate={navigateTo} />
      {:else if currentPage === "settings"}
        <Settings onnavigate={navigateTo} />
      {:else if currentPage === "app-detail"}
        <AppDetail appName={selectedAppName} onnavigate={navigateTo} />
      {/if}
    </div>
  {/key}
</main>

<ExitReminder
  show={showExitReminder}
  {totalTime}
  {topApp}
  {topAppTime}
  {appCount}
  onclose={handleReminderClose}
  onquit={handleQuit}
/>

<style>
  .app-container {
    width: 100%;
    min-height: 100vh;
    padding: 20px;
    max-width: 800px;
    margin: 0 auto;
  }

  .paused-banner {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 10px;
    margin-bottom: 16px;
    background: var(--accent-1);
    border: 2px solid var(--pixel-border);
    font-family: var(--font-pixel);
    font-size: 10px;
    color: var(--text-primary);
    box-shadow: inset 2px 2px 0 rgba(255,255,255,0.1), inset -2px -2px 0 rgba(0,0,0,0.3);
  }

  .top-banner {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 10px;
    margin-bottom: 16px;
    background: var(--accent-4);
    border: 2px solid var(--pixel-border);
    font-family: var(--font-pixel);
    font-size: 10px;
    color: #000;
    box-shadow: inset 2px 2px 0 rgba(255,255,255,0.1), inset -2px -2px 0 rgba(0,0,0,0.3);
  }

  .paused-icon, .top-icon {
    font-size: 16px;
  }

  .resume-btn {
    font-family: var(--font-pixel);
    font-size: 9px;
    padding: 4px 12px;
    background: var(--accent-3);
    color: #000;
    border: 2px solid var(--pixel-border);
    cursor: pointer;
    box-shadow: inset -2px -2px 0 rgba(0,0,0,0.3), inset 2px 2px 0 rgba(255,255,255,0.1);
  }

  .resume-btn:hover {
    background: var(--accent-2);
    color: #fff;
  }

  @media (max-width: 480px) {
    .app-container {
      padding: 12px;
    }
  }
</style>
