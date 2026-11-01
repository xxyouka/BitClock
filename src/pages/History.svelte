<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import PixelHeader from "../components/PixelHeader.svelte";
  import PixelCard from "../components/PixelCard.svelte";
  import PixelNumber from "../components/PixelNumber.svelte";
  import type { DailySummary, AppUsage } from "../stores/stats.svelte";
  import { t } from "../stores/i18n.svelte";
  import { formatCompact, getDayLabel, formatDateShort } from "../utils/format";

  import type { NavPage } from "../types";
  let { onnavigate }: { onnavigate?: (page: NavPage) => void } = $props();

  type Tab = "week" | "month";
  let activeTab: Tab = $state("week");
  let weeklyData: DailySummary[] = $state([]);
  let monthlyData: DailySummary[] = $state([]);
  let appRanking: AppUsage[] = $state([]);
  let loading = $state(true);
  let error: string | null = $state(null);
  let hoveredBar: number | null = $state(null);
  let selectedBar: number | null = $state(null);
  let fetchVersion = $state(0);
  let comparison: {
    current_total: number;
    previous_total: number;
    current_avg: number;
    previous_avg: number;
    current_days: number;
    previous_days: number;
    change_pct: number;
  } | null = $state(null);

  let currentData = $derived(activeTab === "week" ? weeklyData : monthlyData);
  let maxSeconds = $derived(Math.max(...currentData.map((d) => d.total_sec), 1));
  let totalSeconds = $derived(currentData.reduce((sum, d) => sum + d.total_sec, 0));
  let avgSeconds = $derived(currentData.length > 0 ? totalSeconds / currentData.length : 0);
  let selectedDay = $derived(selectedBar !== null && selectedBar < currentData.length ? currentData[selectedBar] : null);
  let selectedDayPct = $derived(selectedDay && avgSeconds > 0 ? ((selectedDay.total_sec / avgSeconds) * 100).toFixed(0) : null);
  let maxDaySeconds = $derived(currentData.length > 0 ? Math.max(...currentData.map((d) => d.total_sec)) : 0);
  let maxDayDate = $derived(currentData.length > 0 ? formatDateShort(currentData.find((d) => d.total_sec === maxDaySeconds)?.date || "") : "");
  let activeDays = $derived(currentData.filter((d) => d.total_sec > 0).length);

  async function fetchComparison(period: string) {
    try {
      comparison = await invoke("get_comparison", { period });
    } catch {
      comparison = null;
    }
  }

  async function fetchData() {
    try {
      weeklyData = await invoke<DailySummary[]>("get_weekly_stats");
      monthlyData = await invoke<DailySummary[]>("get_monthly_stats");
      await fetchAppRanking(activeTab === "week" ? weeklyData : monthlyData);
      await fetchComparison(activeTab === "week" ? "week" : "month");
      loading = false;
    } catch (e) {
      console.error("Failed to fetch stats:", e);
      error = String(e);
      loading = false;
    }
  }

  async function fetchAppRanking(data: DailySummary[]) {
    const version = ++fetchVersion;
    if (data.length === 0) {
      appRanking = [];
      return;
    }

    const startDate = data[data.length - 1].date;
    const endDate = data[0].date;

    try {
      const result = await invoke<AppUsage[]>("get_app_ranking_range", { startDate, endDate });
      if (version === fetchVersion) {
        appRanking = result;
      }
    } catch (e) {
      console.error("Failed to fetch app ranking:", e);
    }
  }

  function handleTabChange(tab: Tab) {
    activeTab = tab;
    hoveredBar = null;
    selectedBar = null;
    fetchAppRanking(tab === "week" ? weeklyData : monthlyData);
    fetchComparison(tab === "week" ? "week" : "month");
  }

  onMount(fetchData);
</script>

<div class="history-page">
  <PixelHeader title={t("history.title")} showBack={true} {onnavigate} />

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
    <!-- Tabs -->
    <div class="tabs">
      <button
        class="tab-btn"
        class:active={activeTab === "week"}
        onclick={() => handleTabChange("week")}
      >
        {t("history.this_week")}
      </button>
      <button
        class="tab-btn"
        class:active={activeTab === "month"}
        onclick={() => handleTabChange("month")}
      >
        {t("history.this_month")}
      </button>
    </div>

    <!-- Chart -->
    <PixelCard>
      <div class="chart-container">
        {#key activeTab}
          <div class="chart">
            {#each currentData as day, i (day.date)}
              <div
                class="chart-column"
                class:selected={selectedBar === i}
                onmouseenter={() => hoveredBar = i}
                onmouseleave={() => hoveredBar = null}
                onclick={() => selectedBar = selectedBar === i ? null : i}
                role="presentation"
              >
                {#if hoveredBar === i}
                  <div class="chart-tooltip">
                    <span class="tooltip-date">{formatDateShort(day.date)}</span>
                    <span class="tooltip-sep">·</span>
                    <span class="tooltip-time">{formatCompact(day.total_sec)}</span>
                    <span class="tooltip-apps">{day.app_count} {t("home.app_count")}</span>
                  </div>
                {/if}
                <div class="chart-bar-wrapper">
                  <div
                    class="chart-bar"
                    style="height: {(day.total_sec / maxSeconds) * 100}%; animation-delay: {i * 60}ms"
                  >
                    <div class="chart-bar-inner"></div>
                  </div>
                </div>
                <div class="chart-label">
                  {activeTab === "week" ? getDayLabel(day.date) : day.date.split("-")[2]}
                </div>
              </div>
            {/each}
          </div>
        {/key}
      </div>
    </PixelCard>

    <!-- Selected day detail -->
    {#if selectedDay}
      <PixelCard>
        <div class="selected-detail">
          <span class="selected-date">{formatDateShort(selectedDay.date)}</span>
          <span class="selected-time">{formatCompact(selectedDay.total_sec)}</span>
          <span class="selected-apps">{selectedDay.app_count} {t("home.app_count")}</span>
          {#if selectedDayPct}
            <span class="selected-pct" class:above={Number(selectedDayPct) > 100}>
              {t("history.daily_avg_pct", { pct: selectedDayPct })}
            </span>
          {/if}
        </div>
      </PixelCard>
    {/if}

    <!-- Summary stats -->
    <div class="summary-grid">
      <PixelCard>
        <div class="summary-item">
          <div class="summary-label">{t("history.total")}</div>
          <PixelNumber value={formatCompact(totalSeconds)} size="medium" />
        </div>
      </PixelCard>

      <PixelCard>
        <div class="summary-item">
          <div class="summary-label">{t("history.daily_avg")}</div>
          <PixelNumber value={formatCompact(avgSeconds)} size="medium" color="success" />
        </div>
      </PixelCard>

      <PixelCard>
        <div class="summary-item">
          <div class="summary-label">{t("history.longest_day")}</div>
          <PixelNumber value={formatCompact(maxDaySeconds)} size="medium" />
          <div class="summary-sub">{maxDayDate}</div>
        </div>
      </PixelCard>

      <PixelCard>
        <div class="summary-item">
          <div class="summary-label">{t("history.active_days")}</div>
          <PixelNumber value={String(activeDays)} size="medium" color="success" />
        </div>
      </PixelCard>
    </div>

    <!-- Comparison -->
    {#if comparison}
      <PixelCard>
        <h3 class="section-title">{t("history.comparison")}</h3>
        <div class="comparison-grid">
          <div class="comp-item">
            <div class="comp-label">{activeTab === "week" ? t("history.this_week_vs_last") : t("history.this_month_vs_last")}</div>
            <div class="comp-change" class:positive={comparison.change_pct > 0} class:negative={comparison.change_pct < 0}>
              {#if comparison.change_pct > 0}
                {t("history.increase")} ({t("history.change_pct", { pct: comparison.change_pct.toFixed(1) })})
              {:else if comparison.change_pct < 0}
                {t("history.decrease")} ({t("history.change_pct", { pct: Math.abs(comparison.change_pct).toFixed(1) })})
              {:else}
                {t("history.no_change")}
              {/if}
            </div>
          </div>
          <div class="comp-details">
            <div class="comp-detail">
              <span class="comp-detail-label">{t("history.daily_avg")}</span>
              <span class="comp-detail-value">{formatCompact(Math.round(comparison.current_avg))}</span>
              <span class="comp-detail-prev">← {formatCompact(Math.round(comparison.previous_avg))}</span>
            </div>
            <div class="comp-detail">
              <span class="comp-detail-label">{t("history.total")}</span>
              <span class="comp-detail-value">{formatCompact(comparison.current_total)}</span>
              <span class="comp-detail-prev">← {formatCompact(comparison.previous_total)}</span>
            </div>
          </div>
        </div>
      </PixelCard>
    {/if}

    <!-- Top apps for selected period -->
    <PixelCard>
      <h3 class="section-title">{t("history.top_apps")}</h3>
      <div class="app-list">
        {#each appRanking.slice(0, 5) as app, i (app.process_name)}
          <div class="app-item">
            <span class="app-rank">#{i + 1}</span>
            <span class="app-name" title={app.process_name}>{app.process_name}</span>
            <span class="app-time">{formatCompact(app.duration_sec)}</span>
          </div>
        {/each}

        {#if appRanking.length === 0}
          <div class="empty-state">{t("home.no_data")}</div>
        {/if}
      </div>
    </PixelCard>
  {/if}
</div>

<style>
  .history-page {
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

  .tabs {
    display: flex;
    gap: 8px;
  }

  .tab-btn {
    font-family: var(--font-pixel);
    font-size: 10px;
    padding: 8px 16px;
    background: var(--bg-secondary);
    color: var(--text-primary);
    border: 2px solid var(--pixel-border);
    cursor: pointer;
    box-shadow: var(--shadow-pixel-3);
  }

  .tab-btn.active {
    background: var(--accent-2);
    color: #fff;
  }

  .tab-btn:hover:not(.active) {
    background: var(--accent-1);
  }

  .chart-container {
    padding: 16px 0;
  }

  .chart {
    display: flex;
    align-items: flex-end;
    justify-content: space-around;
    height: 200px;
    gap: 8px;
  }

  .chart-column {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    height: 100%;
    position: relative;
    cursor: pointer;
  }

  .chart-tooltip {
    position: absolute;
    bottom: calc(100% + 8px);
    left: 50%;
    transform: translateX(-50%);
    background: var(--bg-secondary);
    border: 2px solid var(--accent-2);
    padding: 4px 8px;
    font-family: var(--font-pixel);
    font-size: 8px;
    color: var(--text-primary);
    white-space: nowrap;
    box-shadow: 3px 3px 0 rgba(0,0,0,0.4);
    z-index: 10;
    pointer-events: none;
  }

  .chart-tooltip::after {
    content: "";
    position: absolute;
    top: 100%;
    left: 50%;
    transform: translateX(-50%);
    border: 4px solid transparent;
    border-top-color: var(--accent-2);
  }

  .tooltip-date {
    color: var(--text-secondary);
  }

  .tooltip-sep {
    color: var(--text-secondary);
    margin: 0 2px;
  }

  .tooltip-time {
    color: var(--accent-4);
  }

  .chart-bar-wrapper {
    flex: 1;
    width: 100%;
    display: flex;
    align-items: flex-end;
    justify-content: center;
  }

  .chart-bar {
    width: 70%;
    min-height: 4px;
    background: var(--accent-4);
    border: 2px solid var(--pixel-border);
    position: relative;
    box-shadow:
      inset -2px -2px 0 rgba(0,0,0,0.3),
      inset 2px 2px 0 rgba(255,255,255,0.1);
    animation: growUp 0.4s ease-out both;
    transform-origin: bottom;
  }

  @keyframes growUp {
    from {
      transform: scaleY(0);
      opacity: 0;
    }
    to {
      transform: scaleY(1);
      opacity: 1;
    }
  }

  .chart-bar-inner {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 40%;
    background: linear-gradient(180deg, rgba(255,255,255,0.2) 0%, transparent 100%);
  }

  .chart-label {
    font-family: var(--font-pixel);
    font-size: 8px;
    color: var(--text-secondary);
    margin-top: 8px;
  }

  .chart-column.selected .chart-bar {
    box-shadow: 0 0 8px var(--accent-4);
    border-color: var(--accent-4);
  }

  .chart-column.selected .chart-label {
    color: var(--accent-4);
    font-weight: bold;
  }

  .tooltip-apps {
    display: block;
    font-size: 7px;
    color: var(--text-secondary);
    margin-top: 2px;
  }

  .selected-detail {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 0;
    font-family: var(--font-pixel);
    font-size: 10px;
  }

  .selected-date {
    color: var(--accent-2);
    font-size: 11px;
  }

  .selected-time {
    color: var(--accent-3);
    font-size: 12px;
  }

  .selected-apps {
    color: var(--text-secondary);
    font-size: 9px;
  }

  .selected-pct {
    color: var(--accent-3);
    font-size: 9px;
    margin-left: auto;
  }

  .selected-pct.above {
    color: var(--accent-2);
  }

  .summary-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }

  .summary-item {
    text-align: center;
    padding: 8px 0;
  }

  .summary-label {
    font-family: var(--font-pixel);
    font-size: 10px;
    color: var(--text-secondary);
    margin-bottom: 8px;
    text-transform: uppercase;
  }

  .summary-sub {
    font-family: var(--font-pixel);
    font-size: 8px;
    color: var(--text-secondary);
    margin-top: 4px;
  }

  .section-title {
    font-family: var(--font-pixel);
    font-size: 12px;
    color: var(--accent-2);
    margin-bottom: 12px;
    text-shadow: 1px 1px 0 rgba(0,0,0,0.5);
  }

  .app-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .app-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px;
    background: var(--bg-primary);
    border: 1px solid var(--pixel-border);
  }

  .app-rank {
    font-family: var(--font-number);
    font-size: 16px;
    color: var(--accent-2);
    min-width: 24px;
  }

  .app-name {
    flex: 1;
    font-family: var(--font-pixel);
    font-size: 9px;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .app-time {
    font-family: var(--font-number);
    font-size: 14px;
    color: var(--accent-4);
  }

  .empty-state {
    text-align: center;
    padding: 20px;
    font-family: var(--font-pixel);
    font-size: 10px;
    color: var(--text-secondary);
  }

  .comparison-grid {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 8px 0;
  }

  .comp-item {
    text-align: center;
  }

  .comp-label {
    font-family: var(--font-pixel);
    font-size: 9px;
    color: var(--text-secondary);
    margin-bottom: 4px;
  }

  .comp-change {
    font-family: var(--font-pixel);
    font-size: 11px;
    color: var(--text-primary);
  }

  .comp-change.positive {
    color: var(--accent-2);
  }

  .comp-change.negative {
    color: var(--accent-3);
  }

  .comp-details {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }

  .comp-detail {
    text-align: center;
    padding: 8px;
    background: var(--bg-primary);
    border: 1px solid var(--pixel-border);
  }

  .comp-detail-label {
    display: block;
    font-family: var(--font-pixel);
    font-size: 7px;
    color: var(--text-secondary);
    margin-bottom: 4px;
    text-transform: uppercase;
  }

  .comp-detail-value {
    display: block;
    font-family: var(--font-number);
    font-size: 16px;
    color: var(--accent-3);
  }

  .comp-detail-prev {
    display: block;
    font-family: var(--font-number);
    font-size: 12px;
    color: var(--text-secondary);
    margin-top: 2px;
  }

  @media (max-width: 480px) {
    .summary-grid {
      grid-template-columns: 1fr;
    }

    .chart {
      height: 140px;
    }

    .app-rank {
      display: none;
    }
  }
</style>
