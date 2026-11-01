<script lang="ts">
  let { value, size = "medium", color = "accent" }: {
    value: string;
    size?: "small" | "medium" | "large";
    color?: "accent" | "success";
  } = $props();
</script>

<span class="pixel-number {size} {color}">{value}</span>

<style>
  .pixel-number {
    font-family: var(--font-number);
    text-shadow: 2px 2px 0 rgba(0,0,0,0.5);
    letter-spacing: 2px;
  }

  /* Sizes */
  .small {
    font-size: 20px;
  }

  .medium {
    font-size: 32px;
  }

  .large {
    font-size: 48px;
  }

  /* Colors */
  .accent {
    color: var(--accent-4);
  }

  .success {
    color: var(--accent-3);
  }
</style>
