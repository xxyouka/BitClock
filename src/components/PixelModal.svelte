<script lang="ts">
  import type { Snippet } from "svelte";
  import { t } from "../stores/i18n.svelte";

  let { show = false, title = "", onclose, children }: {
    show?: boolean;
    title?: string;
    onclose?: () => void;
    children?: Snippet;
  } = $props();

  let overlayEl: HTMLDivElement | undefined = $state(undefined);
  let previousFocus: HTMLElement | null = null;

  $effect(() => {
    if (show) {
      previousFocus = document.activeElement as HTMLElement;
      document.body.style.overflow = "hidden";
      requestAnimationFrame(() => {
        overlayEl?.focus();
      });
    } else {
      document.body.style.overflow = "";
      if (previousFocus) {
        previousFocus.focus();
        previousFocus = null;
      }
    }
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onclose?.();
      return;
    }
    if (e.key === "Tab" && overlayEl) {
      const focusable = overlayEl.querySelectorAll<HTMLElement>(
        'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
      );
      if (focusable.length === 0) return;
      const first = focusable[0];
      const last = focusable[focusable.length - 1];
      if (e.shiftKey && document.activeElement === first) {
        e.preventDefault();
        last.focus();
      } else if (!e.shiftKey && document.activeElement === last) {
        e.preventDefault();
        first.focus();
      }
    }
  }
</script>

{#if show}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="modal-overlay"
    bind:this={overlayEl}
    onclick={() => onclose?.()}
    onkeydown={handleKeydown}
    role="dialog"
    aria-modal="true"
    aria-label={title || t("modal.default_label")}
    tabindex="-1"
  >
    <div class="modal-content" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
      {#if title}
        <div class="modal-header">
          <h3 class="modal-title">{title}</h3>
          <button class="modal-close" onclick={() => onclose?.()} aria-label={t("modal.close")}>×</button>
        </div>
      {/if}

      <div class="modal-body">
        {@render children?.()}
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fadeIn 0.2s ease-out;
  }

  .modal-content {
    background: var(--bg-secondary);
    border: 2px solid var(--pixel-border);
    box-shadow:
      inset 2px 2px 0 rgba(255,255,255,0.05),
      inset -2px -2px 0 rgba(0,0,0,0.2),
      8px 8px 0 rgba(0,0,0,0.4);
    max-width: 90vw;
    max-height: 90vh;
    overflow: auto;
    animation: modalSlideIn 0.2s ease-out;
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 2px solid var(--pixel-border);
  }

  .modal-title {
    font-family: var(--font-pixel);
    font-size: 12px;
    color: var(--accent-2);
    text-shadow: 1px 1px 0 rgba(0,0,0,0.5);
  }

  .modal-close {
    font-family: var(--font-pixel);
    font-size: 16px;
    background: none;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 4px 8px;
  }

  .modal-close:hover {
    color: var(--accent-2);
  }

  .modal-body {
    padding: 16px;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  @keyframes modalSlideIn {
    from {
      opacity: 0;
      transform: translateY(-20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
