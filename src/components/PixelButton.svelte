<script lang="ts">
  import type { Snippet } from "svelte";

  let { variant = "primary", size = "medium", disabled = false, onclick, children }: {
    variant?: "primary" | "secondary" | "danger";
    size?: "small" | "medium" | "large";
    disabled?: boolean;
    onclick?: () => void;
    children?: Snippet;
  } = $props();
</script>

<button
  class="pixel-btn {variant} {size}"
  {disabled}
  onclick={() => onclick?.()}
>
  {@render children?.()}
</button>

<style>
  .pixel-btn {
    font-family: var(--font-pixel);
    cursor: pointer;
    position: relative;
    transition: none;
    border: 2px solid var(--pixel-border);
    box-shadow: var(--shadow-pixel-4);
    text-transform: uppercase;
  }

  .pixel-btn:hover:not(:disabled) {
    transform: translate(1px, 1px);
    box-shadow: var(--shadow-pixel-3);
  }

  .pixel-btn:active:not(:disabled) {
    transform: translate(2px, 2px);
    box-shadow: var(--shadow-pixel-2);
  }

  .pixel-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Variants */
  .primary {
    background: var(--accent-1);
    color: var(--text-primary);
  }

  .primary:hover:not(:disabled) {
    background: var(--accent-2);
  }

  .secondary {
    background: var(--bg-secondary);
    color: var(--text-primary);
  }

  .secondary:hover:not(:disabled) {
    background: var(--accent-1);
  }

  .danger {
    background: #c0392b;
    color: #fff;
  }

  .danger:hover:not(:disabled) {
    background: #e74c3c;
  }

  /* Sizes */
  .small {
    font-size: 8px;
    padding: 6px 12px;
  }

  .medium {
    font-size: 10px;
    padding: 8px 16px;
  }

  .large {
    font-size: 12px;
    padding: 12px 24px;
  }
</style>
