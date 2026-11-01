<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { save, open } from "@tauri-apps/plugin-dialog";
  import PixelHeader from "../components/PixelHeader.svelte";
  import PixelCard from "../components/PixelCard.svelte";
  import PixelToggle from "../components/PixelToggle.svelte";
  import PixelButton from "../components/PixelButton.svelte";
  import PixelModal from "../components/PixelModal.svelte";
  import { setSettings, type Settings } from "../stores/settings.svelte";
  import { getThemeSetting, setTheme, type ThemeSetting } from "../stores/theme.svelte";
  import { t, getLocale, setLocale, type Locale } from "../stores/i18n.svelte";

  import type { NavPage } from "../types";
  let { onnavigate }: { onnavigate?: (page: NavPage) => void } = $props();

  let autostart = $state(true);
  let prevAutostart = $state(true);
  let reminderOnExit = $state(true);
  let dailyGoalMinutes = $state(0);
  let retentionDays = $state(90);
  let idleThresholdMinutes = $state(5);
  let breakReminderMinutes = $state(45);
  let customAccentColor = $state("");
  let pomodoroEnabled = $state(false);
  let pomodoroWorkMinutes = $state(25);
  let pomodoroBreakMinutes = $state(5);
  let pomodoroLongBreakMinutes = $state(15);
  let pomodoroCycles = $state(4);
  let saving = $state(false);
  let loaded = $state(false);
  let errorMsg = $state("");
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  let showCleanupModal = $state(false);
  let cleanupResult = $state(0);
  let confirmCleanup = $state(false);
  let confirmTimer: ReturnType<typeof setTimeout> | null = null;
  let exporting = $state(false);
  let backing = $state(false);
  let categories: [string, string][] = $state([]);
  let currentLocale = $state(getLocale());

  const retentionOptions = [30, 60, 90, 180, 365];
  const categoryOptions = [
    { key: "工作", label: t("category.work") },
    { key: "娱乐", label: t("category.entertainment") },
    { key: "社交", label: t("category.social") },
    { key: "开发", label: t("category.development") },
    { key: "设计", label: t("category.design") },
    { key: "其他", label: t("category.other") },
  ];

  onMount(async () => {
    try {
      const s: Settings = await invoke<Settings>("get_settings");
      autostart = s.autostart;
      prevAutostart = s.autostart;
      reminderOnExit = s.reminder_on_exit;
      dailyGoalMinutes = s.daily_goal_minutes ?? 0;
      retentionDays = s.retention_days ?? 90;
      idleThresholdMinutes = s.idle_threshold_minutes ?? 5;
      breakReminderMinutes = s.break_reminder_minutes ?? 45;
      customAccentColor = s.custom_accent_color ?? "";
      pomodoroEnabled = s.pomodoro_enabled ?? false;
      pomodoroWorkMinutes = s.pomodoro_work_minutes ?? 25;
      pomodoroBreakMinutes = s.pomodoro_break_minutes ?? 5;
      pomodoroLongBreakMinutes = s.pomodoro_long_break_minutes ?? 15;
      pomodoroCycles = s.pomodoro_cycles ?? 4;
      setSettings(s);
      applyCustomColor(customAccentColor);
    } catch (e) {
      console.error("Failed to load settings:", e);
    }
    loadCategories();
    loaded = true;
  });

  onDestroy(() => {
    if (debounceTimer) clearTimeout(debounceTimer);
  });

  function scheduleSave() {
    if (!loaded) return;
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(saveSettings, 300);
  }

  async function saveSettings() {
    saving = true;
    errorMsg = "";
    try {
      await invoke("update_settings", {
        settings: {
          autostart,
          reminder_on_exit: reminderOnExit,
          daily_goal_minutes: dailyGoalMinutes,
          retention_days: retentionDays,
          idle_threshold_minutes: idleThresholdMinutes,
          break_reminder_minutes: breakReminderMinutes,
          custom_accent_color: customAccentColor,
          pomodoro_enabled: pomodoroEnabled,
          pomodoro_work_minutes: pomodoroWorkMinutes,
          pomodoro_break_minutes: pomodoroBreakMinutes,
          pomodoro_long_break_minutes: pomodoroLongBreakMinutes,
          pomodoro_cycles: pomodoroCycles,
        },
      });
      if (autostart !== prevAutostart) {
        try {
          await invoke("toggle_autostart", { enable: autostart });
          prevAutostart = autostart;
        } catch (regErr) {
          errorMsg = t("error.registry");
          console.error("Registry error:", regErr);
        }
      }
      setSettings({
        autostart,
        reminder_on_exit: reminderOnExit,
        daily_goal_minutes: dailyGoalMinutes,
        retention_days: retentionDays,
        idle_threshold_minutes: idleThresholdMinutes,
        break_reminder_minutes: breakReminderMinutes,
        custom_accent_color: customAccentColor,
        pomodoro_enabled: pomodoroEnabled,
        pomodoro_work_minutes: pomodoroWorkMinutes,
        pomodoro_break_minutes: pomodoroBreakMinutes,
        pomodoro_long_break_minutes: pomodoroLongBreakMinutes,
        pomodoro_cycles: pomodoroCycles,
      });
    } catch (e) {
      errorMsg = t("error.save_settings");
      console.error("Failed to save settings:", e);
    }
    saving = false;
  }

  function handleThemeChange(newTheme: ThemeSetting) {
    setTheme(newTheme);
  }

  function applyCustomColor(color: string) {
    if (color && /^#[0-9A-Fa-f]{6}$/.test(color)) {
      document.documentElement.style.setProperty('--accent-2', color);
    } else {
      document.documentElement.style.removeProperty('--accent-2');
    }
  }

  function handleAccentColorChange(e: Event) {
    const input = e.target as HTMLInputElement;
    customAccentColor = input.value;
    applyCustomColor(customAccentColor);
    scheduleSave();
  }

  function resetAccentColor() {
    customAccentColor = "";
    applyCustomColor("");
    scheduleSave();
  }

  async function handleLocaleChange(locale: Locale) {
    setLocale(locale);
    currentLocale = locale;
    // Sync locale to Rust side for tray menu language
    try {
      await invoke("set_locale", { locale });
    } catch {
      // Silent fail — tray will use previous locale until restart
    }
  }

  function handleAutostartChange() {
    autostart = !autostart;
    scheduleSave();
  }

  function handleReminderChange() {
    reminderOnExit = !reminderOnExit;
    scheduleSave();
  }

  function handleGoalChange(e: Event) {
    const input = e.target as HTMLInputElement;
    const val = parseInt(input.value) || 0;
    dailyGoalMinutes = Math.max(0, Math.min(1440, val));
    input.value = String(dailyGoalMinutes);
    scheduleSave();
  }

  function handleRetentionChange(e: Event) {
    retentionDays = parseInt((e.target as HTMLSelectElement).value) || 90;
    scheduleSave();
  }

  function handleIdleThresholdChange(e: Event) {
    const input = e.target as HTMLInputElement;
    const val = parseInt(input.value) || 5;
    idleThresholdMinutes = Math.max(1, Math.min(60, val));
    input.value = String(idleThresholdMinutes);
    scheduleSave();
  }

  function handleBreakReminderChange(e: Event) {
    const input = e.target as HTMLInputElement;
    const val = parseInt(input.value) || 45;
    breakReminderMinutes = Math.max(10, Math.min(120, val));
    input.value = String(breakReminderMinutes);
    scheduleSave();
  }

  async function handleCleanup() {
    if (!confirmCleanup) {
      confirmCleanup = true;
      if (confirmTimer) clearTimeout(confirmTimer);
      confirmTimer = setTimeout(() => { confirmCleanup = false; }, 3000);
      return;
    }
    if (confirmTimer) { clearTimeout(confirmTimer); confirmTimer = null; }
    confirmCleanup = false;
    try {
      cleanupResult = await invoke<number>("cleanup_old_records");
      showCleanupModal = true;
    } catch (e) {
      console.error("Cleanup failed:", e);
    }
  }

  async function loadCategories() {
    try {
      categories = await invoke<[string, string][]>("get_all_categories");
    } catch {
      // Silent fail
    }
  }

  async function handleCategoryChange(processName: string, category: string) {
    try {
      await invoke("set_app_category", { processName, category });
      categories = categories.map(([p, c]) => [p, p === processName ? category : c]);
    } catch (e) {
      console.error("Failed to update category:", e);
    }
  }

  async function handleExport(format: "json" | "csv" = "json") {
    exporting = true;
    try {
      const isCsv = format === "csv";
      const path = await save({
        defaultPath: isCsv ? "bitclock-export.csv" : "bitclock-export.json",
        filters: isCsv
          ? [{ name: "CSV", extensions: ["csv"] }]
          : [{ name: "JSON", extensions: ["json"] }],
      });
      if (path) {
        const command = isCsv ? "export_csv_to_file" : "export_to_file";
        const count = await invoke<number>(command, { path });
        errorMsg = "";
        alert(t("settings.export_done", { count }));
      }
    } catch (e) {
      console.error("Export failed:", e);
      errorMsg = t("error.export");
    }
    exporting = false;
  }

  async function handleBackup() {
    backing = true;
    try {
      const path = await save({
        defaultPath: "bitclock-backup.db",
        filters: [{ name: "Database", extensions: ["db"] }],
      });
      if (path) {
        await invoke("backup_database", { destPath: path });
        alert(t("settings.backup_done"));
      }
    } catch (e) {
      console.error("Backup failed:", e);
      errorMsg = t("error.backup");
    }
    backing = false;
  }

  async function handleRestore() {
    try {
      const path = await open({
        filters: [{ name: "Database", extensions: ["db"] }],
      });
      if (path) {
        if (!confirm(t("settings.restore_confirm"))) return;
        await invoke("restore_database", { srcPath: path });
        alert(t("settings.restore_done"));
      }
    } catch (e) {
      console.error("Restore failed:", e);
      errorMsg = t("error.restore");
    }
  }
</script>

<div class="settings-page">
  <PixelHeader title={t("settings.title")} showBack={true} showSettings={false} {onnavigate} />

  <!-- Theme -->
  <PixelCard>
    <h3 class="section-title">{t("settings.appearance")}</h3>
    <div class="theme-selector">
      <span class="theme-label">{t("settings.theme")}</span>
      <div class="theme-buttons">
        <button
          class="theme-btn"
          class:active={getThemeSetting() === "dark"}
          onclick={() => handleThemeChange("dark")}
        >
          <span class="theme-icon">🌙</span>
          {t("settings.dark")}
        </button>
        <button
          class="theme-btn"
          class:active={getThemeSetting() === "light"}
          onclick={() => handleThemeChange("light")}
        >
          <span class="theme-icon">☀</span>
          {t("settings.light")}
        </button>
        <button
          class="theme-btn"
          class:active={getThemeSetting() === "auto"}
          onclick={() => handleThemeChange("auto")}
        >
          <span class="theme-icon">⏰</span>
          {t("settings.auto")}
        </button>
      </div>
    </div>
    <div class="color-customizer">
      <span class="theme-label">{t("settings.accent_color")}</span>
      <div class="color-input-row">
        <input
          type="color"
          class="color-picker"
          value={customAccentColor || "#e94560"}
          oninput={handleAccentColorChange}
        />
        <span class="color-value">{customAccentColor || t("settings.default")}</span>
        {#if customAccentColor}
          <button class="color-reset" onclick={resetAccentColor}>{t("settings.reset")}</button>
        {/if}
      </div>
    </div>
    <div class="language-selector">
      <span class="theme-label">{t("settings.language")}</span>
      <div class="theme-buttons">
        <button
          class="theme-btn"
          class:active={currentLocale === "zh"}
          onclick={() => handleLocaleChange("zh")}
        >
          中文
        </button>
        <button
          class="theme-btn"
          class:active={currentLocale === "en"}
          onclick={() => handleLocaleChange("en")}
        >
          English
        </button>
      </div>
    </div>
  </PixelCard>

  <!-- Startup -->
  <PixelCard>
    <h3 class="section-title">{t("settings.startup")}</h3>
    <PixelToggle checked={autostart} label={t("settings.autostart")} ontoggle={handleAutostartChange} />
  </PixelCard>

  <!-- Reminder -->
  <PixelCard>
    <h3 class="section-title">{t("settings.reminder")}</h3>
    <PixelToggle checked={reminderOnExit} label={t("settings.exit_reminder")} ontoggle={handleReminderChange} />
  </PixelCard>

  <!-- Idle detection -->
  <PixelCard>
    <h3 class="section-title">{t("settings.idle_detection")}</h3>
    <div class="goal-setting">
      <span class="goal-label">{t("settings.idle_threshold")}</span>
      <div class="goal-input-row">
        <input
          type="number"
          class="goal-input"
          min="1"
          max="60"
          value={idleThresholdMinutes}
          oninput={handleIdleThresholdChange}
          placeholder="5"
        />
        <span class="goal-unit">{t("settings.minutes")}</span>
        <span class="goal-hint">1-60 {t("settings.minutes")}</span>
      </div>
    </div>
  </PixelCard>

  <!-- Break reminder -->
  <PixelCard>
    <h3 class="section-title">{t("settings.break_reminder")}</h3>
    <div class="goal-setting">
      <span class="goal-label">{t("settings.break_interval")}</span>
      <div class="goal-input-row">
        <input
          type="number"
          class="goal-input"
          min="10"
          max="120"
          value={breakReminderMinutes}
          oninput={handleBreakReminderChange}
          placeholder="45"
        />
        <span class="goal-unit">{t("settings.minutes")}</span>
        <span class="goal-hint">10-120 {t("settings.minutes")}</span>
      </div>
    </div>
  </PixelCard>

  <!-- Pomodoro -->
  <PixelCard>
    <h3 class="section-title">🍅 {t("settings.pomodoro")}</h3>
    <PixelToggle checked={pomodoroEnabled} label={t("settings.pomodoro_enable")} ontoggle={() => { pomodoroEnabled = !pomodoroEnabled; scheduleSave(); }} />
    {#if pomodoroEnabled}
      <div class="goal-setting" style="margin-top: 12px;">
        <div class="goal-setting-row">
          <span class="goal-label">{t("settings.pomodoro_work")}</span>
          <div class="goal-input-row">
            <input type="number" class="goal-input" min="1" max="60" value={pomodoroWorkMinutes}
              oninput={(e) => { pomodoroWorkMinutes = Math.max(1, Math.min(60, parseInt((e.target as HTMLInputElement).value) || 25)); scheduleSave(); }} />
            <span class="goal-unit">{t("settings.minutes")}</span>
          </div>
        </div>
        <div class="goal-setting-row">
          <span class="goal-label">{t("settings.pomodoro_break")}</span>
          <div class="goal-input-row">
            <input type="number" class="goal-input" min="1" max="30" value={pomodoroBreakMinutes}
              oninput={(e) => { pomodoroBreakMinutes = Math.max(1, Math.min(30, parseInt((e.target as HTMLInputElement).value) || 5)); scheduleSave(); }} />
            <span class="goal-unit">{t("settings.minutes")}</span>
          </div>
        </div>
        <div class="goal-setting-row">
          <span class="goal-label">{t("settings.pomodoro_long_break")}</span>
          <div class="goal-input-row">
            <input type="number" class="goal-input" min="5" max="60" value={pomodoroLongBreakMinutes}
              oninput={(e) => { pomodoroLongBreakMinutes = Math.max(5, Math.min(60, parseInt((e.target as HTMLInputElement).value) || 15)); scheduleSave(); }} />
            <span class="goal-unit">{t("settings.minutes")}</span>
          </div>
        </div>
        <div class="goal-setting-row">
          <span class="goal-label">{t("settings.pomodoro_cycles")}</span>
          <div class="goal-input-row">
            <input type="number" class="goal-input" min="2" max="10" value={pomodoroCycles}
              oninput={(e) => { pomodoroCycles = Math.max(2, Math.min(10, parseInt((e.target as HTMLInputElement).value) || 4)); scheduleSave(); }} />
            <span class="goal-hint">{t("settings.pomodoro_cycles_hint", { count: pomodoroCycles })}</span>
          </div>
        </div>
      </div>
    {/if}
  </PixelCard>

  <!-- Daily goal -->
  <PixelCard>
    <h3 class="section-title">{t("settings.daily_goal")}</h3>
    <div class="goal-setting">
      <span class="goal-label">{t("settings.usage_limit")}</span>
      <div class="goal-input-row">
        <input
          type="number"
          class="goal-input"
          min="0"
          max="1440"
          value={dailyGoalMinutes}
          oninput={handleGoalChange}
          placeholder="0"
        />
        <span class="goal-unit">{t("settings.minutes")}</span>
        <span class="goal-hint">{t("settings.disable_hint")}</span>
      </div>
    </div>
  </PixelCard>

  <!-- Data management -->
  <PixelCard>
    <h3 class="section-title">{t("settings.data")}</h3>
    <div class="data-row">
      <span class="data-label">{t("settings.retention_days")}</span>
      <select class="retention-select" value={retentionDays} onchange={handleRetentionChange}>
        {#each retentionOptions as days}
          <option value={days}>{days} {t("settings.days")}</option>
        {/each}
      </select>
    </div>
    <div class="data-actions">
      <p class="data-info">{t("settings.auto_cleanup", { days: retentionDays })}</p>
      <PixelButton variant={confirmCleanup ? "danger" : "primary"} size="small" onclick={handleCleanup}>
        {confirmCleanup ? t("settings.confirm_cleanup") : t("settings.cleanup_now")}
      </PixelButton>
    </div>
    <div class="data-actions" style="margin-top: 8px;">
      <p class="data-info">{t("settings.export_records")}</p>
      <div class="export-buttons">
        <PixelButton variant="primary" size="small" onclick={() => handleExport("json")} disabled={exporting}>
          {exporting ? t("settings.exporting") : 'JSON'}
        </PixelButton>
        <PixelButton variant="primary" size="small" onclick={() => handleExport("csv")} disabled={exporting}>
          {exporting ? t("settings.exporting") : 'CSV'}
        </PixelButton>
      </div>
    </div>
    <div class="data-actions" style="margin-top: 8px;">
      <p class="data-info">{t("settings.backup_db")}</p>
      <PixelButton variant="primary" size="small" onclick={handleBackup} disabled={backing}>
        {backing ? t("settings.backing") : t("settings.backup")}
      </PixelButton>
    </div>
    <div class="data-actions" style="margin-top: 8px;">
      <p class="data-info">{t("settings.restore_hint")}</p>
      <PixelButton variant="danger" size="small" onclick={handleRestore}>
        {t("settings.restore")}
      </PixelButton>
    </div>
  </PixelCard>

  <!-- Categories -->
  <PixelCard>
    <h3 class="section-title">{t("settings.categories")}</h3>
    <p class="data-info" style="margin-bottom: 12px;">{t("settings.categories_hint")}</p>
    <div class="category-list">
      {#each categories as [process, cat]}
        <div class="category-item">
          <span class="category-process" title={process}>{process}</span>
          <select
            class="category-select"
            value={cat}
            onchange={(e) => handleCategoryChange(process, (e.target as HTMLSelectElement).value)}
          >
            {#each categoryOptions as option}
              <option value={option.key}>{option.label}</option>
            {/each}
          </select>
        </div>
      {/each}
      {#if categories.length === 0}
        <div class="empty-state">{t("settings.categories_empty")}</div>
      {/if}
    </div>
  </PixelCard>

  <!-- About -->
  <PixelCard>
    <h3 class="section-title">{t("settings.about")}</h3>
    <div class="about">
      <div class="about-logo">
        <span class="logo-icon">■</span>
        <span class="logo-text">{t("app.name")}</span>
      </div>
      <div class="version">v1.1.0</div>
      <div class="description">{t("app.description")}</div>
    </div>
  </PixelCard>

  <!-- Status -->
  {#if errorMsg}
    <button class="status-indicator error" onclick={() => errorMsg = ""}>
      {errorMsg} ({t("settings.click_to_close")})
    </button>
  {:else if saving}
    <div class="status-indicator saving">
      <span class="animate-blink">{t("settings.saving")}</span>
    </div>
  {/if}

  <PixelModal show={showCleanupModal} title={t("settings.cleanup_done")} onclose={() => showCleanupModal = false}>
    <div class="cleanup-result">
      <p>{t("settings.cleanup_result", { count: cleanupResult })}</p>
    </div>
  </PixelModal>
</div>

<style>
  .settings-page {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .section-title {
    font-family: var(--font-pixel);
    font-size: 12px;
    color: var(--accent-2);
    margin-bottom: 12px;
    text-shadow: 1px 1px 0 rgba(0,0,0,0.5);
  }

  .theme-selector {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .theme-label {
    font-family: var(--font-pixel);
    font-size: 10px;
    color: var(--text-primary);
  }

  .theme-buttons {
    display: flex;
    gap: 8px;
  }

  .theme-btn {
    font-family: var(--font-pixel);
    font-size: 9px;
    padding: 8px 16px;
    background: var(--bg-primary);
    color: var(--text-primary);
    border: 2px solid var(--pixel-border);
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 6px;
    box-shadow: var(--shadow-pixel-3);
  }

  .theme-btn.active {
    background: var(--accent-2);
    color: #fff;
  }

  .theme-btn:hover:not(.active) {
    background: var(--accent-1);
  }

  .theme-icon {
    font-size: 14px;
  }

  .color-customizer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: 16px;
    padding-top: 16px;
    border-top: 1px solid var(--pixel-border);
  }

  .color-input-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .color-picker {
    width: 32px;
    height: 32px;
    border: 2px solid var(--pixel-border);
    cursor: pointer;
    background: none;
    padding: 0;
  }

  .color-picker::-webkit-color-swatch-wrapper {
    padding: 2px;
  }

  .color-picker::-webkit-color-swatch {
    border: none;
  }

  .color-value {
    font-family: var(--font-number);
    font-size: 14px;
    color: var(--text-secondary);
    min-width: 60px;
  }

  .color-reset {
    font-family: var(--font-pixel);
    font-size: 8px;
    padding: 4px 8px;
    background: var(--accent-1);
    color: var(--text-primary);
    border: 2px solid var(--pixel-border);
    cursor: pointer;
  }

  .color-reset:hover {
    background: var(--accent-2);
  }

  .language-selector {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: 16px;
    padding-top: 16px;
    border-top: 1px solid var(--pixel-border);
  }

  .data-actions {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .export-buttons {
    display: flex;
    gap: 8px;
  }

  .goal-setting {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .goal-setting-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }

  .goal-label {
    font-family: var(--font-pixel);
    font-size: 10px;
    color: var(--text-primary);
  }

  .goal-input-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .goal-input {
    width: 80px;
    font-family: var(--font-number);
    font-size: 18px;
    padding: 4px 8px;
    background: var(--bg-primary);
    color: var(--text-primary);
    border: 2px solid var(--pixel-border);
    outline: none;
    text-align: center;
  }

  .goal-input:focus {
    border-color: var(--accent-2);
  }

  .goal-unit {
    font-family: var(--font-pixel);
    font-size: 9px;
    color: var(--text-secondary);
  }

  .goal-hint {
    font-family: var(--font-pixel);
    font-size: 7px;
    color: var(--text-secondary);
    margin-left: auto;
  }

  .data-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 12px;
  }

  .data-label {
    font-family: var(--font-pixel);
    font-size: 10px;
    color: var(--text-primary);
  }

  .retention-select {
    font-family: var(--font-pixel);
    font-size: 9px;
    padding: 4px 8px;
    background: var(--bg-primary);
    color: var(--text-primary);
    border: 2px solid var(--pixel-border);
    cursor: pointer;
    outline: none;
  }

  .retention-select:focus {
    border-color: var(--accent-2);
  }

  .data-info {
    font-family: var(--font-pixel);
    font-size: 9px;
    color: var(--text-secondary);
  }

  .category-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    max-height: 300px;
    overflow-y: auto;
  }

  .category-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 8px;
    background: var(--bg-primary);
    border: 1px solid var(--pixel-border);
  }

  .category-process {
    flex: 1;
    font-family: var(--font-pixel);
    font-size: 9px;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .category-select {
    font-family: var(--font-pixel);
    font-size: 9px;
    padding: 4px 8px;
    background: var(--bg-secondary);
    color: var(--text-primary);
    border: 2px solid var(--pixel-border);
    cursor: pointer;
    outline: none;
  }

  .category-select:focus {
    border-color: var(--accent-2);
  }

  .empty-state {
    text-align: center;
    padding: 20px;
    font-family: var(--font-pixel);
    font-size: 10px;
    color: var(--text-secondary);
  }

  .about {
    text-align: center;
    padding: 16px 0;
  }

  .about-logo {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    margin-bottom: 8px;
  }

  .logo-icon {
    font-size: 24px;
    color: var(--accent-3);
  }

  .logo-text {
    font-family: var(--font-pixel);
    font-size: 16px;
    color: var(--accent-2);
    text-shadow: 2px 2px 0 rgba(0,0,0,0.5);
  }

  .version {
    font-family: var(--font-number);
    font-size: 18px;
    color: var(--text-secondary);
    margin-bottom: 8px;
  }

  .description {
    font-family: var(--font-pixel);
    font-size: 9px;
    color: var(--text-secondary);
  }

  .status-indicator {
    position: fixed;
    bottom: 20px;
    right: 20px;
    font-family: var(--font-pixel);
    font-size: 10px;
    background: var(--bg-secondary);
    padding: 8px 16px;
    box-shadow: 4px 4px 0 rgba(0,0,0,0.3);
  }

  .status-indicator.saving {
    color: var(--accent-3);
    border: 2px solid var(--pixel-border);
  }

  .status-indicator.error {
    color: var(--accent-2);
    border: 2px solid var(--accent-2);
    cursor: pointer;
  }

  .cleanup-result {
    font-family: var(--font-pixel);
    font-size: 10px;
    color: var(--text-primary);
    text-align: center;
    padding: 16px 0;
  }

  @media (max-width: 480px) {
    .theme-buttons {
      flex-direction: column;
    }
  }
</style>
