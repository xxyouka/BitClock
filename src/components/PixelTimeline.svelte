<script lang="ts">
  import { formatCompact } from "../utils/format";

  let { data }: { data: { hour: number; duration_sec: number }[] } = $props();

  let hoveredHour: number | null = $state(null);

  let maxSeconds = $derived(Math.max(...data.map((d) => d.duration_sec), 1));
  let currentHour = $derived(new Date().getHours());
</script>

{#if data.length > 0}
  <div class="timeline">
    {#each data as slot}
      <div
        class="timeline-col"
        class:active={slot.hour === currentHour}
        class:has-data={slot.duration_sec > 0}
        onmouseenter={() => hoveredHour = slot.hour}
        onmouseleave={() => hoveredHour = null}
        role="presentation"
      >
        {#if hoveredHour === slot.hour}
          <div class="timeline-tooltip">
            {String(slot.hour).padStart(2, "0")}:00 · {formatCompact(slot.duration_sec)}
          </div>
        {/if}
        <div class="timeline-bar-wrapper">
          <div
            class="timeline-bar"
            style="height: {(slot.duration_sec / maxSeconds) * 100}%; animation-delay: {slot.hour * 30}ms"
          ></div>
        </div>
        {#if slot.hour % 3 === 0}
          <div class="timeline-label">{String(slot.hour).padStart(2, "0")}</div>
        {:else}
          <div class="timeline-label-space"></div>
        {/if}
      </div>
    {/each}
  </div>
{/if}

<style>
  .timeline {
    display: flex;
    align-items: flex-end;
    gap: 2px;
    height: 64px;
    padding: 0 2px;
  }

  .timeline-col {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    height: 100%;
    position: relative;
    cursor: pointer;
  }

  .timeline-tooltip {
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

  .timeline-bar-wrapper {
    flex: 1;
    width: 100%;
    display: flex;
    align-items: flex-end;
    justify-content: center;
  }

  .timeline-bar {
    width: 80%;
    min-height: 1px;
    background: var(--accent-1);
    border: 1px solid var(--pixel-border);
    animation: timelineGrow 0.3s ease-out both;
    transform-origin: bottom;
  }

  .timeline-col.has-data .timeline-bar {
    background: var(--accent-3);
  }

  .timeline-col.active .timeline-bar {
    background: var(--accent-4);
    box-shadow: 0 0 4px var(--accent-4);
  }

  .timeline-label {
    font-family: var(--font-number);
    font-size: 10px;
    color: var(--text-secondary);
    margin-top: 2px;
  }

  .timeline-label-space {
    height: 12px;
  }

  @keyframes timelineGrow {
    from {
      transform: scaleY(0);
      opacity: 0;
    }
    to {
      transform: scaleY(1);
      opacity: 1;
    }
  }
</style>
