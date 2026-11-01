<script lang="ts">
  import PixelButton from "./PixelButton.svelte";
  import { t } from "../stores/i18n.svelte";
  import {
    getPomodoroState,
    startPomodoro,
    stopPomodoro,
    skipPhase,
    formatPomodoroTime,
  } from "../stores/pomodoro.svelte";

  let state = $derived(getPomodoroState());

  let phaseLabel = $derived(
    state.phase === "work" ? t("pomodoro.work") :
    state.phase === "break" ? t("pomodoro.break") :
    state.phase === "long-break" ? t("pomodoro.long_break") :
    t("pomodoro.title")
  );

  let phaseColor = $derived(
    state.phase === "work" ? "var(--accent-2)" :
    state.phase === "break" ? "var(--accent-3)" :
    state.phase === "long-break" ? "#9b59b6" :
    "var(--text-secondary)"
  );
</script>

{#if state.enabled}
  <div class="pomodoro" class:active={state.phase !== "idle"}>
    <div class="pomodoro-header">
      <span class="pomodoro-icon">🍅</span>
      <span class="pomodoro-title">{t("pomodoro.title")}</span>
    </div>

    {#if state.phase !== "idle"}
      <div class="pomodoro-timer" style="color: {phaseColor}">
        {formatPomodoroTime(state.remainingSeconds)}
      </div>
      <div class="pomodoro-phase" style="color: {phaseColor}">
        {phaseLabel}
      </div>
      <div class="pomodoro-cycle">
        {t("pomodoro.cycle", { n: state.cycleCount + 1 })}
      </div>
      <div class="pomodoro-actions">
        <PixelButton variant="secondary" size="small" onclick={skipPhase}>
          {t("pomodoro.skip")}
        </PixelButton>
        <PixelButton variant="danger" size="small" onclick={stopPomodoro}>
          {t("pomodoro.stop")}
        </PixelButton>
      </div>
    {:else}
      <div class="pomodoro-idle">
        <PixelButton variant="primary" size="small" onclick={startPomodoro}>
          {t("pomodoro.start")}
        </PixelButton>
      </div>
    {/if}
  </div>
{/if}

<style>
  .pomodoro {
    padding: 12px 0;
    text-align: center;
  }

  .pomodoro.active {
    padding: 16px 0;
  }

  .pomodoro-header {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    margin-bottom: 8px;
  }

  .pomodoro-icon {
    font-size: 16px;
  }

  .pomodoro-title {
    font-family: var(--font-pixel);
    font-size: 10px;
    color: var(--text-secondary);
    text-transform: uppercase;
  }

  .pomodoro-timer {
    font-family: var(--font-number);
    font-size: 36px;
    margin: 8px 0;
    text-shadow: 2px 2px 0 rgba(0,0,0,0.3);
  }

  .pomodoro-phase {
    font-family: var(--font-pixel);
    font-size: 10px;
    margin-bottom: 4px;
  }

  .pomodoro-cycle {
    font-family: var(--font-pixel);
    font-size: 8px;
    color: var(--text-secondary);
    margin-bottom: 12px;
  }

  .pomodoro-actions {
    display: flex;
    justify-content: center;
    gap: 8px;
  }

  .pomodoro-idle {
    margin-top: 8px;
  }
</style>
