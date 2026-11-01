<script lang="ts">
  let { checked, label, ontoggle }: {
    checked: boolean;
    label: string;
    ontoggle?: () => void;
  } = $props();
</script>

<div class="toggle-container">
  <span class="toggle-label">{label}</span>
  <button
    class="toggle"
    class:active={checked}
    onclick={() => ontoggle?.()}
    role="switch"
    aria-checked={checked}
    aria-label={label}
  >
    <span class="toggle-knob"></span>
  </button>
</div>

<style>
  .toggle-container {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 0;
  }

  .toggle-label {
    font-family: var(--font-pixel);
    font-size: 10px;
    color: var(--text-primary);
  }

  .toggle {
    width: 48px;
    height: 24px;
    background: var(--bg-primary);
    border: 2px solid var(--pixel-border);
    cursor: pointer;
    position: relative;
    box-shadow: inset 2px 2px 0 rgba(0,0,0,0.3);
    transition: none;
  }

  .toggle.active {
    background: var(--accent-3);
  }

  .toggle-knob {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 16px;
    height: 16px;
    background: var(--text-primary);
    border: 2px solid var(--pixel-border);
    box-shadow:
      inset -1px -1px 0 rgba(0,0,0,0.3),
      inset 1px 1px 0 rgba(255,255,255,0.2);
    transition: transform 0.1s;
  }

  .toggle.active .toggle-knob {
    transform: translateX(24px);
  }
</style>
