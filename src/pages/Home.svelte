<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { listen } from "@tauri-apps/api/event";
  import { sendNotification, requestPermission } from "@tauri-apps/plugin-notification";
  import PixelHeader from "../components/PixelHeader.svelte";
  import PixelCard from "../components/PixelCard.svelte";
  import PixelNumber from "../components/PixelNumber.svelte";
  import PixelBar from "../components/PixelBar.svelte";
  import PixelButton from "../components/PixelButton.svelte";
  import PixelModal from "../components/PixelModal.svelte";
  import PixelMiniChart from "../components/PixelMiniChart.svelte";
  import PixelPieChart from "../components/PixelPieChart.svelte";
  import PixelCategoryStats from "../components/PixelCategoryStats.svelte";
  import PixelTimeline from "../components/PixelTimeline.svelte";
  import PixelPomodoro from "../components/PixelPomodoro.svelte";
  import { initPomodoro } from "../stores/pomodoro.svelte";
  import { getTodayStats, setTodayStats, getFormattedTotalTime, type TodayStats, type CurrentApp, type DailySummary } from "../stores/stats.svelte";
  import type { Settings } from "../stores/settings.svelte";
  import { t } from "../stores/i18n.svelte";
  import { formatCompact } from "../utils/format";

  import type { NavPage } from "../types";
  let { onnavigate }: { onnavigate?: (page: NavPage, appName?: string) => void } = $props();

  let weeklyData: DailySummary[] = $state([]);
  let categoryStats: [string, number][] = $state([]);
  let hourlyData: { hour: number; duration_sec: number }[] = $state([]);
  let loading = $state(true);
  let error: string | null = $state(null);
  let currentAppName = $state("");
  let currentAppTitle = $state("");
  let currentAppDuration = $state("00:00:00");
  let dailyGoalMinutes = $state(0);
  let breakReminderMinutes = $state(45);
  let lastBreakTime = $state(Date.now());
  let showBreakReminder = $state(false);

  let statsInterval: ReturnType<typeof setInterval> | null = null;
  let breakCheckInterval: ReturnType<typeof setInterval> | null = null;
  let unlistenForeground: (() => void) | null = null;
  let unlistenVis: (() => void) | null = null;
  let lastFetchTime = 0;
  const FETCH_THROTTLE_MS = 5000;
  let showGoalReminder = $state(false);
  let goalNotified = $state(false);
  let goalWarningNotified = $state(false);
  let goalWarning = $state(false);
  let breakNotified = $state(false);

  let goalProgress = $derived.by(() => {
    if (dailyGoalMinutes <= 0) return 0;
    const goalSec = dailyGoalMinutes * 60;
    return Math.min(100, (getTodayStats().total_sec / goalSec) * 100);
  });
  let goalExceeded = $derived(dailyGoalMinutes > 0 && getTodayStats().total_sec >= dailyGoalMinutes * 60);
  let goalApproaching = $derived(dailyGoalMinutes > 0 && goalProgress >= 80 && !goalExceeded);

  $effect(() => {
    if (goalExceeded && !goalNotified) {
      showGoalReminder = true;
      goalNotified = true;
      goalWarningNotified = true;
      sendNotification({
        title: t("app.name"),
        body: t("home.goal_exceeded") + ` (${dailyGoalMinutes} min)`,
      });
    } else if (goalApproaching && !goalWarningNotified) {
      goalWarning = true;
      goalWarningNotified = true;
      sendNotification({
        title: t("app.name"),
        body: t("home.goal_approaching") + ` (${Math.round(goalProgress)}%)`,
      });
    }
  });

  async function fetchStats() {
    try {
      const stats: TodayStats = await invoke<TodayStats>("get_today_stats");
      setTodayStats(stats);
      loading = false;
      error = null;
      // Notify tray with today's total seconds for tooltip
      getCurrentWindow().emit("stats-updated", String(stats.total_sec));
    } catch (e) {
      error = String(e);
      loading = false;
    }
  }

  async function fetchWeeklyData() {
    try {
      weeklyData = await invoke<DailySummary[]>("get_weekly_stats");
    } catch {
      // Silent fail
    }
  }

  async function fetchCategoryStats() {
    try {
      categoryStats = await invoke<[string, number][]>("get_category_stats");
    } catch {
      // Silent fail
    }
  }

  async function fetchHourlyData() {
    try {
      hourlyData = await invoke<{ hour: number; duration_sec: number }[]>("get_hourly_stats");
    } catch {
      // Silent fail
    }
  }

  async function checkWeeklyReport() {
    try {
      const lastReport = localStorage.getItem("bitclock-last-weekly-report");
      const now = new Date();
      const dayOfWeek = now.getDay();
      // Send weekly report on Monday (day 1) if not sent today
      if (dayOfWeek === 1) {
        const today = now.toISOString().slice(0, 10);
        if (lastReport === today) return;

        const weekly: DailySummary[] = await invoke("get_weekly_stats");
        if (weekly.length === 0) return;

        const totalSec = weekly.reduce((sum, d) => sum + d.total_sec, 0);
        const avgSec = Math.round(totalSec / weekly.length);
        sendNotification({
          title: t("app.name"),
          body: t("home.weekly_report", { total: formatCompact(totalSec), avg: formatCompact(avgSec) }),
        });
        localStorage.setItem("bitclock-last-weekly-report", today);
      }
    } catch {
      // Silent fail
    }
  }

  async function fetchSettings() {
    try {
      const s = await invoke<Settings>("get_settings");
      dailyGoalMinutes = s.daily_goal_minutes ?? 0;
      breakReminderMinutes = s.break_reminder_minutes ?? 45;
      initPomodoro(s);
    } catch {
      // Silent fail
    }
  }

  function checkBreakReminder() {
    if (breakReminderMinutes <= 0 || breakNotified) return;
    const elapsed = (Date.now() - lastBreakTime) / 1000 / 60;
    if (elapsed >= breakReminderMinutes) {
      showBreakReminder = true;
      breakNotified = true;
      sendNotification({
        title: t("app.name"),
        body: t("home.break_reminder"),
      });
    }
  }

  function handleBreakTaken() {
    lastBreakTime = Date.now();
    showBreakReminder = false;
    breakNotified = false;
  }

  function updateCurrentApp(app: CurrentApp | null) {
    if (app) {
      currentAppName = app.process_name;
      currentAppTitle = app.window_title;
      currentAppDuration = formatCompact(app.duration_sec);
    } else {
      currentAppName = "";
      currentAppTitle = "";
    }
  }

  async function loadCurrentApp() {
    try {
      const app = await invoke<CurrentApp | null>("get_current_app");
      updateCurrentApp(app);
    } catch {
      // Silent fail
    }
  }

  function startPolling() {
    if (statsInterval) return;
    fetchStats();
    statsInterval = setInterval(fetchStats, 15000);
    // Check break reminder every minute
    breakCheckInterval = setInterval(checkBreakReminder, 60000);
  }

  function stopPolling() {
    if (statsInterval) { clearInterval(statsInterval); statsInterval = null; }
    if (breakCheckInterval) { clearInterval(breakCheckInterval); breakCheckInterval = null; }
  }

  onMount(async () => {
    fetchWeeklyData();
    fetchSettings();
    fetchCategoryStats();
    fetchHourlyData();
    loadCurrentApp();
    checkWeeklyReport();
    startPolling();

    // Request notification permission
    try {
      await requestPermission();
    } catch {
      // Notification permission denied or unavailable
    }

    // Listen for real-time foreground window changes from Rust tracker
    unlistenForeground = await listen<CurrentApp | null>("foreground-changed", (event) => {
      updateCurrentApp(event.payload);
      // Throttled stats refresh to avoid excessive DB queries
      const now = Date.now();
      if (now - lastFetchTime >= FETCH_THROTTLE_MS) {
        lastFetchTime = now;
        fetchStats();
      }
    });

    try {
      const win = getCurrentWindow();
      unlistenVis = await win.onVisibilityChange((visible) => {
        if (visible) {
          startPolling();
          loadCurrentApp();
        } else {
          stopPolling();
        }
      });
    } catch {
      // Fallback: always poll if visibility API unavailable
    }
  });

  onDestroy(() => {
    stopPolling();
    unlistenForeground?.();
    unlistenVis?.();
  });
</script>

<div class="home-page">
  <PixelHeader title={t("app.name")} showBack={false} {onnavigate} />

  {#if loading}
    <div class="loading">
      <span class="animate-blink">{t("home.loading")}</span>
    </div>
  {:else if error}
    <PixelCard>
      <div class="error">
        <span class="error-icon" aria-hidden="true">⚠</span>
        <p>{error}</p>
      </div>
    </PixelCard>
  {:else}
    <!-- Today's total time -->
    <PixelCard>
      <div class="total-time">
        <div class="label">{t("home.today_usage")}</div>
        <PixelNumber value={getFormattedTotalTime()} size="large" />
        <button class="refresh-btn" onclick={fetchStats} title={t("home.refresh")} aria-label={t("home.refresh")}>
          ↻
        </button>
      </div>
      {#if dailyGoalMinutes > 0}
        <div class="goal-section">
          <div class="goal-header">
            <span class="goal-label">{t("home.daily_goal")}</span>
            <span class="goal-value" class:exceeded={goalExceeded} class:approaching={goalApproaching}>
              {formatCompact(getTodayStats().total_sec)} / {dailyGoalMinutes}m
            </span>
          </div>
          <div class="goal-track">
            <div
              class="goal-fill"
              class:exceeded={goalExceeded}
              class:approaching={goalApproaching}
              style="width: {goalProgress}%"
            ></div>
          </div>
          {#if goalApproaching}
            <div class="goal-warning">
              ⚠ {t("home.goal_approaching")}
            </div>
          {/if}
        </div>
      {/if}
      {#if weeklyData.length > 0}
        <div class="mini-chart-section">
          <div class="mini-chart-label">{t("home.weekly_trend")}</div>
          <PixelMiniChart data={weeklyData} />
        </div>
      {/if}
      {#if hourlyData.length > 0}
        <div class="mini-chart-section">
          <div class="mini-chart-label">{t("home.timeline")}</div>
          <PixelTimeline data={hourlyData} />
        </div>
      {/if}
    </PixelCard>

    <!-- Current app (real-time) -->
    {#if currentAppName}
      <PixelCard>
        <div class="current-app">
          <div class="current-label">{t("home.current_app")}</div>
          <div class="current-name" title={currentAppName}>{currentAppName}</div>
          <div class="current-title" title={currentAppTitle}>{currentAppTitle}</div>
          <div class="current-duration">
            {t("home.duration_prefix")}<PixelNumber value={currentAppDuration} size="small" />
          </div>
        </div>
      </PixelCard>
    {/if}

    <!-- Pomodoro -->
    <PixelCard>
      <PixelPomodoro />
    </PixelCard>

    <!-- App ranking -->
    <PixelCard>
      <div class="ranking-header">
        <h2 class="section-title">{t("home.app_ranking")}</h2>
        <span class="app-count">{getTodayStats().app_count} {t("home.app_count")}</span>
      </div>

      <div class="ranking-list">
        {#each getTodayStats().app_ranking as app, i (app.process_name)}
          <button class="ranking-item-btn" onclick={() => onnavigate?.("app-detail", app.process_name)} title={t("home.click_app_detail")}>
            <PixelBar
              percentage={app.percentage}
              label={app.process_name}
              value={formatCompact(app.duration_sec)}
              color={i === 0 ? "accent" : i < 3 ? "success" : "default"}
            />
          </button>
        {/each}

        {#if getTodayStats().app_ranking.length === 0}
          <div class="empty-state">
            <span class="empty-icon" aria-hidden="true">📊</span>
            <p>{t("home.no_data")}</p>
          </div>
        {/if}
      </div>
    </PixelCard>

    <!-- Pie chart -->
    {#if getTodayStats().app_ranking.length > 0}
      <PixelCard>
        <h2 class="section-title">{t("home.usage_distribution")}</h2>
        <div class="pie-chart-wrapper">
          <PixelPieChart data={getTodayStats().app_ranking} />
        </div>
      </PixelCard>
    {/if}

    <!-- Category stats -->
    {#if categoryStats.length > 0}
      <PixelCard>
        <h2 class="section-title">{t("home.category_stats")}</h2>
        <PixelCategoryStats data={categoryStats} />
      </PixelCard>
    {/if}

    <!-- Stats link -->
    <div class="stats-link">
      <PixelButton variant="primary" onclick={() => onnavigate?.("history")}>
        {t("home.view_history")}
      </PixelButton>
    </div>
  {/if}

  <!-- Goal reminder modal -->
  <PixelModal show={showGoalReminder} onClose={() => showGoalReminder = false}>
    <div class="goal-reminder">
      <div class="goal-reminder-icon">🎉</div>
      <h3 class="goal-reminder-title">{t("home.goal_exceeded")}</h3>
      <p class="goal-reminder-text">
        {t("home.goal_exceeded_text")}
      </p>
      <p class="goal-reminder-sub">
        {t("home.goal_exceeded_sub", { minutes: dailyGoalMinutes })}
      </p>
      <PixelButton variant="primary" onclick={() => showGoalReminder = false}>
        {t("home.known")}
      </PixelButton>
    </div>
  </PixelModal>

  <!-- Break reminder modal -->
  <PixelModal show={showBreakReminder} onClose={() => showBreakReminder = false}>
    <div class="goal-reminder">
      <div class="goal-reminder-icon">☕</div>
      <h3 class="goal-reminder-title">{t("home.break_reminder")}</h3>
      <p class="goal-reminder-text">
        {t("home.break_reminder_text")}
      </p>
      <p class="goal-reminder-sub">
        {t("home.break_reminder_sub")}
      </p>
      <div class="goal-reminder-actions">
        <PixelButton variant="secondary" onclick={() => showBreakReminder = false}>
          {t("home.snooze")}
        </PixelButton>
        <PixelButton variant="primary" onclick={handleBreakTaken}>
          {t("home.i_took_break")}
        </PixelButton>
      </div>
    </div>
  </PixelModal>
</div>

<style>
  .home-page {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .loading {
    text-align: center;
    padding: 40px;
    font-family: var(--font-pixel);
    font-size: 12px;
    color: var(--text-secondary);
  }

  .error {
    text-align: center;
    padding: 20px;
    color: var(--accent-2);
  }

  .error-icon {
    font-size: 32px;
    display: block;
    margin-bottom: 8px;
  }

  .total-time {
    text-align: center;
    padding: 20px 0;
    position: relative;
  }

  .refresh-btn {
    position: absolute;
    top: 20px;
    right: 0;
    font-family: var(--font-pixel);
    font-size: 16px;
    background: var(--bg-secondary);
    color: var(--text-secondary);
    border: 2px solid var(--pixel-border);
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    box-shadow: var(--shadow-pixel-3);
    transition: transform 0.2s;
  }

  .refresh-btn:hover {
    background: var(--accent-1);
    color: var(--text-primary);
    transform: rotate(180deg);
  }

  .goal-section {
    margin-top: 12px;
    padding-top: 12px;
    border-top: 1px solid var(--pixel-border);
  }

  .goal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 8px;
  }

  .goal-label {
    font-family: var(--font-pixel);
    font-size: 8px;
    color: var(--text-secondary);
    text-transform: uppercase;
  }

  .goal-value {
    font-family: var(--font-number);
    font-size: 14px;
    color: var(--accent-3);
  }

  .goal-value.exceeded {
    color: var(--accent-2);
  }

  .goal-value.approaching {
    color: var(--accent-4);
  }

  .goal-track {
    height: 8px;
    background: var(--bg-primary);
    border: 2px solid var(--pixel-border);
    box-shadow: inset 1px 1px 0 rgba(0,0,0,0.3);
    overflow: hidden;
  }

  .goal-fill {
    height: 100%;
    background: var(--accent-3);
    transition: width 0.5s ease;
  }

  .goal-fill.exceeded {
    background: var(--accent-2);
  }

  .goal-fill.approaching {
    background: var(--accent-4);
  }

  .goal-warning {
    font-family: var(--font-pixel);
    font-size: 9px;
    color: var(--accent-4);
    margin-top: 8px;
    text-align: center;
    animation: blinkSoft 2s infinite;
  }

  @keyframes blinkSoft {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.6; }
  }

  .mini-chart-section {
    margin-top: 12px;
    padding-top: 12px;
    border-top: 1px solid var(--pixel-border);
  }

  .mini-chart-label {
    font-family: var(--font-pixel);
    font-size: 8px;
    color: var(--text-secondary);
    text-transform: uppercase;
    margin-bottom: 8px;
  }

  .label {
    font-family: var(--font-pixel);
    font-size: 12px;
    color: var(--text-secondary);
    margin-bottom: 12px;
    text-transform: uppercase;
  }

  .current-app {
    padding: 8px 0;
  }

  .current-label {
    font-family: var(--font-pixel);
    font-size: 10px;
    color: var(--text-secondary);
    margin-bottom: 8px;
    text-transform: uppercase;
  }

  .current-name {
    font-family: var(--font-pixel);
    font-size: 12px;
    color: var(--accent-3);
    margin-bottom: 4px;
  }

  .current-title {
    font-family: var(--font-pixel);
    font-size: 9px;
    color: var(--text-secondary);
    margin-bottom: 8px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .current-duration {
    display: flex;
    align-items: center;
    gap: 8px;
    font-family: var(--font-pixel);
    font-size: 10px;
    color: var(--text-primary);
  }

  .ranking-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
  }

  .section-title {
    font-family: var(--font-pixel);
    font-size: 12px;
    color: var(--accent-2);
    text-shadow: 1px 1px 0 rgba(0,0,0,0.5);
  }

  .app-count {
    font-family: var(--font-pixel);
    font-size: 9px;
    color: var(--text-secondary);
  }

  .ranking-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .ranking-item-btn {
    display: block;
    width: 100%;
    background: none;
    border: none;
    padding: 0;
    cursor: pointer;
    text-align: left;
    transition: transform 0.1s;
  }

  .ranking-item-btn:hover {
    transform: translateX(2px);
  }

  .ranking-item-btn:active {
    transform: translateX(1px);
  }

  .empty-state {
    text-align: center;
    padding: 20px;
    color: var(--text-secondary);
  }

  .empty-icon {
    font-size: 32px;
    display: block;
    margin-bottom: 8px;
  }

  .stats-link {
    text-align: center;
    padding: 8px 0;
  }

  .pie-chart-wrapper {
    display: flex;
    justify-content: center;
    padding: 8px 0;
  }

  .goal-reminder {
    text-align: center;
    padding: 8px 0;
  }

  .goal-reminder-icon {
    font-size: 48px;
    margin-bottom: 12px;
  }

  .goal-reminder-title {
    font-family: var(--font-pixel);
    font-size: 14px;
    color: var(--accent-3);
    margin-bottom: 12px;
    text-shadow: 1px 1px 0 rgba(0,0,0,0.5);
  }

  .goal-reminder-text {
    font-family: var(--font-pixel);
    font-size: 10px;
    color: var(--text-primary);
    margin-bottom: 8px;
    line-height: 1.6;
  }

  .goal-reminder-sub {
    font-family: var(--font-pixel);
    font-size: 9px;
    color: var(--text-secondary);
    margin-bottom: 16px;
    line-height: 1.6;
  }

  .goal-reminder-actions {
    display: flex;
    justify-content: center;
    gap: 12px;
  }
</style>
