<script lang="ts">
  let { percentage, label, value, color = "default" }: {
    percentage: number;
    label: string;
    value: string;
    color?: "accent" | "success" | "default";
  } = $props();
</script>

<div class="bar-container">
  <div class="bar-label" title={label}>{label}</div>
  <div class="bar-track">
    <div
      class="bar-fill {color}"
      style="width: {Math.min(100, Math.max(0, percentage))}%"
    >
      <div class="bar-inner-shadow"></div>
    </div>
  </div>
  <div class="bar-value">{value}</div>
</div>

<style>
  .bar-container {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 8px;
  }

  .bar-label {
    font-family: var(--font-pixel);
    font-size: 8px;
    color: var(--text-primary);
    min-width: 80px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .bar-track {
    flex: 1;
    height: 16px;
    background: var(--bg-primary);
    border: 2px solid var(--pixel-border);
    box-shadow: inset 2px 2px 0 rgba(0,0,0,0.3);
    position: relative;
    overflow: hidden;
  }

  .bar-fill {
    height: 100%;
    position: relative;
    transition: width 0.5s ease;
    image-rendering: pixelated;
  }

  .bar-inner-shadow {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 50%;
    background: linear-gradient(180deg, rgba(255,255,255,0.2) 0%, transparent 100%);
  }

  .accent {
    background: var(--accent-4);
  }

  .success {
    background: var(--accent-3);
  }

  .default {
    background: var(--accent-1);
  }

  .bar-value {
    font-family: var(--font-number);
    font-size: 14px;
    color: var(--text-secondary);
    min-width: 60px;
    text-align: right;
  }
</style>
