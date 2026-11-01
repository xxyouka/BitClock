<script lang="ts">
  import type { Snippet } from "svelte";

  let { padding = true, children }: {
    padding?: boolean;
    children?: Snippet;
  } = $props();
</script>

<div class="pixel-card" class:padding>
  {@render children?.()}
</div>

<style>
  .pixel-card {
    background: var(--bg-secondary);
    border: 2px solid var(--pixel-border);
    box-shadow:
      inset 2px 2px 0 rgba(255,255,255,0.05),
      inset -2px -2px 0 rgba(0,0,0,0.2),
      6px 6px 0 rgba(0,0,0,0.3);
    animation: slideIn 0.3s ease-out forwards;
    animation-iteration-count: 1;
  }

  .padding {
    padding: 16px;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
