<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import PixelHeader from "../components/PixelHeader.svelte";
  import PixelCard from "../components/PixelCard.svelte";
  import PixelNumber from "../components/PixelNumber.svelte";
  import PixelMiniChart from "../components/PixelMiniChart.svelte";
  import { t } from "../stores/i18n.svelte";
  import { formatCompact } from "../utils/format";
  import type { NavPage } from "../types";

  let { appName, onnavigate }: {
    appName: string;
    onnavigate?: (page: NavPage) => void;
  } = $props();

  interface AppDetailData {
    process_name: string;
    daily_trend: { date: string; total_sec: number; app_count: number }[];
    window_titles: [string, number][];
    total_sec: number;
    avg_daily_sec: number;
  }

  let detail: AppDetailData | null = $state(null);
  let loading = $state(true);
  let error: string | null = $state(null);

  onMount(async () => {
    try {
      detail = await invoke<AppDetailData>("get_app_detail", { processName: appName, days: 30 });
      loading = false;
    } catch (e) {
      error = String(e);
      loading = false;
    }
  });
</script>

<div class="app-detail-page">
  <PixelHeader title={appName} showBack={true} {onnavigate} />

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
  {:else if detail}
    <!-- Summary -->
    <PixelCard>
      <div class="summary">
        <div class="summary-item">
          <div class="summary-label">{t("history.total")}</div>
          <PixelNumber value={formatCompact(detail.total_sec)} size="medium" />
        </div>
        <div class="summary-item">
          <div class="summary-label">{t("history.daily_avg")}</div>
          <PixelNumber value={formatCompact(Math.round(detail.avg_daily_sec))} size="medium" color="success" />
        </div>
      </div>
    </PixelCard>

    <!-- Daily trend (last 7 days) -->
    {#if detail.daily_trend.length > 0}
      <PixelCard>
        <h3 class="section-title">{t("app_detail.daily_trend")}</h3>
        <PixelMiniChart data={detail.daily_trend.slice(0, 7)} />
      </PixelCard>
    {/if}

    <!-- Daily trend table -->
    {#if detail.daily_trend.length > 0}
      <PixelCard>
        <h3 class="section-title">{t("app_detail.history")}</h3>
        <div class="trend-table">
          {#each detail.daily_trend as day}
            <div class="trend-row">
              <span class="trend-date">{day.date}</span>
              <div class="trend-bar-track">
                <div
                  class="trend-bar-fill"
                  style="width: {Math.min(100, (day.total_sec / Math.max(...detail.daily_trend.map(d => d.total_sec), 1)) * 100)}%"
                ></div>
              </div>
              <span class="trend-time">{formatCompact(day.total_sec)}</span>
            </div>
          {/each}
        </div>
      </PixelCard>
    {/if}

    <!-- Window titles -->
    {#if detail.window_titles.length > 0}
      <PixelCard>
        <h3 class="section-title">{t("app_detail.window_titles")}</h3>
        <div class="title-list">
          {#each detail.window_titles as [title, seconds]}
            <div class="title-item">
              <span class="title-text" title={title}>{title || t("app_detail.no_title")}</span>
              <span class="title-time">{formatCompact(seconds)}</span>
            </div>
          {/each}
        </div>
      </PixelCard>
    {/if}
  {/if}
</div>

<style>
  .app-detail-page {
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

  .summary {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    padding: 8px 0;
  }

  .summary-item {
    text-align: center;
  }

  .summary-label {
    font-family: var(--font-pixel);
    font-size: 10px;
    color: var(--text-secondary);
    margin-bottom: 8px;
    text-transform: uppercase;
  }

  .section-title {
    font-family: var(--font-pixel);
    font-size: 12px;
    color: var(--accent-2);
    margin-bottom: 12px;
    text-shadow: 1px 1px 0 rgba(0,0,0,0.5);
  }

  .trend-table {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .trend-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .trend-date {
    font-family: var(--font-number);
    font-size: 14px;
    color: var(--text-secondary);
    min-width: 80px;
  }

  .trend-bar-track {
    flex: 1;
    height: 8px;
    background: var(--bg-primary);
    border: 1px solid var(--pixel-border);
    overflow: hidden;
  }

  .trend-bar-fill {
    height: 100%;
    background: var(--accent-3);
    transition: width 0.3s ease;
  }

  .trend-time {
    font-family: var(--font-number);
    font-size: 14px;
    color: var(--accent-4);
    min-width: 60px;
    text-align: right;
  }

  .title-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .title-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px 8px;
    background: var(--bg-primary);
    border: 1px solid var(--pixel-border);
  }

  .title-text {
    flex: 1;
    font-family: var(--font-pixel);
    font-size: 8px;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .title-time {
    font-family: var(--font-number);
    font-size: 14px;
    color: var(--text-secondary);
    min-width: 50px;
    text-align: right;
  }

  @media (max-width: 480px) {
    .summary {
      grid-template-columns: 1fr;
    }
  }
</style>
