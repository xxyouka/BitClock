<script lang="ts">
  import PixelModal from "./PixelModal.svelte";
  import PixelNumber from "./PixelNumber.svelte";
  import PixelButton from "./PixelButton.svelte";
  import { t } from "../stores/i18n.svelte";
  let { show = false, totalTime = "00:00:00", topApp = "", topAppTime = "", appCount = 0, onclose, onquit }: {
    show?: boolean;
    totalTime?: string;
    topApp?: string;
    topAppTime?: string;
    appCount?: number;
    onclose?: () => void;
    onquit?: () => void;
  } = $props();

  let messages = $derived([
    t("exit.greeting"),
    t("exit.greeting_alt1"),
    t("exit.greeting_alt2"),
    t("exit.greeting_alt3"),
    t("exit.greeting_alt4"),
  ]);
  let message = $state("");

  $effect(() => {
    if (show) {
      message = messages[Math.floor(Math.random() * messages.length)];
    }
  });

</script>

<PixelModal {show} title={t("exit.title")} {onclose}>
  <div class="reminder-content">
    <div class="emoji" aria-hidden="true">☺</div>

    <div class="message">{message}</div>

    <div class="stat-label">{t("exit.usage_summary")}</div>

    <div class="time-display">
      <PixelNumber value={totalTime} size="large" />
    </div>

    {#if topApp}
      <div class="top-app">
        {t("exit.most_used")}<span class="app-name">{topApp}</span>
        {#if topAppTime}
          <span class="app-time">({topAppTime})</span>
        {/if}
      </div>
    {/if}

    <div class="app-count">
      {t("exit.app_count")}<span class="count">{appCount}</span> {t("exit.apps")}
    </div>

    <div class="tip">{t("exit.tip")}</div>
  </div>

  <div class="modal-footer">
    <PixelButton variant="secondary" onclick={() => onclose?.()}>
      {t("exit.cancel")}
    </PixelButton>
    <PixelButton variant="primary" onclick={() => onquit?.()}>
      {t("exit.ok")}
    </PixelButton>
  </div>
</PixelModal>

<style>
  .reminder-content {
    text-align: center;
    padding: 20px 0;
    min-width: 300px;
  }

  .emoji {
    font-size: 48px;
    margin-bottom: 16px;
    animation: bounce 2s infinite;
  }

  .message {
    font-family: var(--font-pixel);
    font-size: 14px;
    color: var(--accent-2);
    margin-bottom: 20px;
    text-shadow: 2px 2px 0 rgba(0,0,0,0.5);
  }

  .stat-label {
    font-family: var(--font-pixel);
    font-size: 10px;
    color: var(--text-secondary);
    margin-bottom: 12px;
  }

  .time-display {
    margin-bottom: 20px;
  }

  .top-app {
    font-family: var(--font-pixel);
    font-size: 10px;
    color: var(--text-primary);
    margin-bottom: 8px;
  }

  .app-name {
    color: var(--accent-3);
  }

  .app-time {
    color: var(--text-secondary);
    font-size: 9px;
  }

  .app-count {
    font-family: var(--font-pixel);
    font-size: 10px;
    color: var(--text-secondary);
    margin-bottom: 16px;
  }

  .count {
    color: var(--accent-4);
    font-family: var(--font-number);
    font-size: 18px;
  }

  .tip {
    font-family: var(--font-pixel);
    font-size: 9px;
    color: var(--accent-3);
    animation: blinkSoft 2s infinite;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    padding-top: 12px;
    border-top: 2px solid var(--pixel-border);
    margin-top: 16px;
  }

  @keyframes bounce {
    0%, 100% {
      transform: translateY(0);
    }
    50% {
      transform: translateY(-10px);
    }
  }

  @keyframes blinkSoft {
    0%, 100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }
</style>
