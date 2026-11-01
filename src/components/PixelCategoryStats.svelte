<script lang="ts">
  import { formatCompact } from "../utils/format";
  import { t } from "../stores/i18n.svelte";

  let { data }: { data: [string, number][] } = $props();

  const CATEGORY_KEY_MAP: Record<string, string> = {
    "工作": "category.work",
    "娱乐": "category.entertainment",
    "社交": "category.social",
    "开发": "category.development",
    "设计": "category.design",
    "其他": "category.other",
  };

  function getCategoryLabel(key: string): string {
    return t(CATEGORY_KEY_MAP[key] || "category.other");
  }

  const categoryIcons: Record<string, string> = {
    "工作": "💼",
    "娱乐": "🎮",
    "社交": "💬",
    "开发": "💻",
    "设计": "🎨",
    "其他": "📁",
  };

  const categoryColors: Record<string, string> = {
    "工作": "var(--accent-3)",
    "娱乐": "var(--accent-2)",
    "社交": "var(--accent-4)",
    "开发": "#9b59b6",
    "设计": "#3498db",
    "其他": "var(--accent-1)",
  };

  let totalSeconds = $derived(data.reduce((sum, [_, sec]) => sum + sec, 0));
</script>

{#if data.length > 0}
  <div class="category-stats">
    {#each data as [category, seconds]}
      <div class="category-row">
        <span class="category-icon">{categoryIcons[category] || "📁"}</span>
        <span class="category-name">{getCategoryLabel(category)}</span>
        <div class="category-bar-track">
          <div
            class="category-bar-fill"
            style="width: {totalSeconds > 0 ? (seconds / totalSeconds) * 100 : 0}%; background: {categoryColors[category] || 'var(--accent-1)'}"
          ></div>
        </div>
        <span class="category-time">{formatCompact(seconds)}</span>
      </div>
    {/each}
  </div>
{/if}

<style>
  .category-stats {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .category-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .category-icon {
    font-size: 16px;
    width: 24px;
    text-align: center;
  }

  .category-name {
    font-family: var(--font-pixel);
    font-size: 9px;
    color: var(--text-primary);
    min-width: 32px;
  }

  .category-bar-track {
    flex: 1;
    height: 12px;
    background: var(--bg-primary);
    border: 1px solid var(--pixel-border);
    overflow: hidden;
  }

  .category-bar-fill {
    height: 100%;
    transition: width 0.5s ease;
  }

  .category-time {
    font-family: var(--font-number);
    font-size: 14px;
    color: var(--text-secondary);
    min-width: 50px;
    text-align: right;
  }
</style>
