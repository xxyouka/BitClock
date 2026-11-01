<script lang="ts">
  import type { DailySummary } from "../stores/stats.svelte";
  import { formatCompact, getDayLabel, formatDateShort } from "../utils/format";

  let { data }: { data: DailySummary[] } = $props();

  let hoveredIndex: number | null = $state(null);

  let maxSeconds = $derived(Math.max(...data.map((d) => d.total_sec), 1));
</script>

{#if data.length > 0}
  <div class="mini-chart">
    {#each data as day, i (day.date)}
      <div
        class="mini-col"
        onmouseenter={() => hoveredIndex = i}
        onmouseleave={() => hoveredIndex = null}
        role="presentation"
      >
        {#if hoveredIndex === i}
          <div class="mini-tooltip">
            {formatDateShort(day.date)} · {formatCompact(day.total_sec)}
          </div>
        {/if}
        <div class="mini-bar-wrapper">
          <div
            class="mini-bar"
            class:today={i === 0}
            style="height: {(day.total_sec / maxSeconds) * 100}%; animation-delay: {i * 50}ms"
          ></div>
        </div>
        <div class="mini-label">{getDayLabel(day.date)}</div>
      </div>
    {/each}
  </div>
{/if}

<style>
  .mini-chart {
    display: flex;
    align-items: flex-end;
    gap: 4px;
    height: 48px;
    padding: 0 4px;
  }

  .mini-col {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    height: 100%;
    position: relative;
    cursor: pointer;
  }

  .mini-tooltip {
    position: absolute;
    bottom: calc(100% + 6px);
    left: 50%;
    transform: translateX(-50%);
    background: var(--bg-secondary);
    border: 2px solid var(--accent-2);
    padding: 2px 6px;
    font-family: var(--font-pixel);
    font-size: 7px;
    color: var(--text-primary);
    white-space: nowrap;
    box-shadow: 2px 2px 0 rgba(0,0,0,0.4);
    z-index: 10;
    pointer-events: none;
  }

  .mini-bar-wrapper {
    flex: 1;
    width: 100%;
    display: flex;
    align-items: flex-end;
    justify-content: center;
  }

  .mini-bar {
    width: 80%;
    min-height: 2px;
    background: var(--accent-1);
    border: 1px solid var(--pixel-border);
    animation: miniGrow 0.3s ease-out both;
    transform-origin: bottom;
  }

  .mini-bar.today {
    background: var(--accent-4);
  }

  @keyframes miniGrow {
    from {
      transform: scaleY(0);
      opacity: 0;
    }
    to {
      transform: scaleY(1);
      opacity: 1;
    }
  }

  .mini-label {
    font-family: var(--font-pixel);
    font-size: 6px;
    color: var(--text-secondary);
    margin-top: 2px;
  }
</style>
