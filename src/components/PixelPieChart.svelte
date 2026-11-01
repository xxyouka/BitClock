<script lang="ts">
  import type { AppUsage } from "../stores/stats.svelte";
  import { formatCompact } from "../utils/format";

  let { data, size = 160 }: { data: AppUsage[]; size?: number } = $props();

  let hoveredIndex: number | null = $state(null);

  const colors = [
    "var(--accent-4)",
    "var(--accent-3)",
    "var(--accent-2)",
    "var(--accent-1)",
    "#9b59b6",
    "#3498db",
    "#1abc9c",
    "#e67e22",
    "#e74c3c",
    "#2ecc71",
  ];

  let segments = $derived.by(() => {
    if (data.length === 0) return [];
    const total = data.reduce((sum, d) => sum + d.duration_sec, 0);
    if (total === 0) return [];

    let currentAngle = 0;
    return data.slice(0, 10).map((app, i) => {
      const percentage = (app.duration_sec / total) * 100;
      const angle = (percentage / 100) * 360;
      const startAngle = currentAngle;
      currentAngle += angle;
      return {
        ...app,
        percentage,
        startAngle,
        angle,
        color: colors[i % colors.length],
      };
    });
  });

  function polarToCartesian(cx: number, cy: number, r: number, angle: number) {
    const rad = ((angle - 90) * Math.PI) / 180;
    return {
      x: cx + r * Math.cos(rad),
      y: cy + r * Math.sin(rad),
    };
  }

  function describeArc(cx: number, cy: number, r: number, startAngle: number, endAngle: number) {
    const start = polarToCartesian(cx, cy, r, endAngle);
    const end = polarToCartesian(cx, cy, r, startAngle);
    const largeArcFlag = endAngle - startAngle <= 180 ? 0 : 1;
    return `M ${cx} ${cy} L ${start.x} ${start.y} A ${r} ${r} 0 ${largeArcFlag} 0 ${end.x} ${end.y} Z`;
  }

  let svgPaths = $derived(
    segments.map((seg, i) => ({
      d: describeArc(size / 2, size / 2, size / 2 - 4, seg.startAngle, seg.startAngle + seg.angle - 1),
      fill: seg.color,
      index: i,
    }))
  );
</script>

{#if data.length > 0}
  <div class="pie-chart-container">
    <svg class="pie-chart" width={size} height={size} viewBox="0 0 {size} {size}">
      {#each svgPaths as path}
        <path
          d={path.d}
          fill={path.fill}
          class:highlighted={hoveredIndex === path.index}
          onmouseenter={() => hoveredIndex = path.index}
          onmouseleave={() => hoveredIndex = null}
          role="presentation"
        />
      {/each}
    </svg>

    {#if hoveredIndex !== null && hoveredIndex < segments.length}
      <div class="pie-tooltip">
        <span class="tooltip-name">{segments[hoveredIndex].process_name}</span>
        <span class="tooltip-time">{formatCompact(segments[hoveredIndex].duration_sec)}</span>
        <span class="tooltip-pct">{segments[hoveredIndex].percentage.toFixed(1)}%</span>
      </div>
    {/if}
  </div>

  <div class="pie-legend">
    {#each segments.slice(0, 5) as seg, i}
      <div
        class="legend-item"
        class:active={hoveredIndex === i}
        onmouseenter={() => hoveredIndex = i}
        onmouseleave={() => hoveredIndex = null}
        role="presentation"
      >
        <span class="legend-color" style="background: {seg.color}"></span>
        <span class="legend-name" title={seg.process_name}>{seg.process_name}</span>
        <span class="legend-pct">{seg.percentage.toFixed(0)}%</span>
      </div>
    {/each}
  </div>
{/if}

<style>
  .pie-chart-container {
    position: relative;
    display: inline-block;
  }

  .pie-chart {
    cursor: pointer;
  }

  .pie-chart path {
    stroke: var(--bg-secondary);
    stroke-width: 2;
    transition: opacity 0.2s;
  }

  .pie-chart path.highlighted {
    opacity: 0.8;
    stroke-width: 3;
    stroke: var(--text-primary);
  }

  .pie-tooltip {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background: var(--bg-secondary);
    border: 2px solid var(--accent-2);
    padding: 8px 12px;
    text-align: center;
    box-shadow: 3px 3px 0 rgba(0,0,0,0.4);
    pointer-events: none;
    z-index: 10;
  }

  .tooltip-name {
    display: block;
    font-family: var(--font-pixel);
    font-size: 8px;
    color: var(--text-primary);
    margin-bottom: 4px;
    max-width: 100px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .tooltip-time {
    display: block;
    font-family: var(--font-number);
    font-size: 16px;
    color: var(--accent-4);
  }

  .tooltip-pct {
    display: block;
    font-family: var(--font-pixel);
    font-size: 8px;
    color: var(--text-secondary);
  }

  .pie-legend {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-top: 12px;
  }

  .legend-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px 8px;
    cursor: pointer;
    transition: background 0.2s;
  }

  .legend-item:hover,
  .legend-item.active {
    background: var(--bg-primary);
  }

  .legend-color {
    width: 12px;
    height: 12px;
    border: 1px solid var(--pixel-border);
    flex-shrink: 0;
  }

  .legend-name {
    flex: 1;
    font-family: var(--font-pixel);
    font-size: 8px;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .legend-pct {
    font-family: var(--font-number);
    font-size: 14px;
    color: var(--text-secondary);
    min-width: 30px;
    text-align: right;
  }
</style>
