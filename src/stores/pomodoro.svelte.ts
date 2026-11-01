import { sendNotification } from "@tauri-apps/plugin-notification";
import { t } from "./i18n.svelte";

export type PomodoroPhase = "work" | "break" | "long-break" | "idle";

let enabled = $state(false);
let phase = $state<PomodoroPhase>("idle");
let remainingSeconds = $state(0);
let cycleCount = $state(0);
let workMinutes = $state(25);
let breakMinutes = $state(5);
let longBreakMinutes = $state(15);
let cyclesBeforeLong = $state(4);
let timerHandle: ReturnType<typeof setInterval> | null = null;

export function getPomodoroState() {
  return { enabled, phase, remainingSeconds, cycleCount };
}

export function initPomodoro(settings: {
  pomodoro_enabled: boolean;
  pomodoro_work_minutes: number;
  pomodoro_break_minutes: number;
  pomodoro_long_break_minutes: number;
  pomodoro_cycles: number;
}) {
  enabled = settings.pomodoro_enabled;
  workMinutes = settings.pomodoro_work_minutes;
  breakMinutes = settings.pomodoro_break_minutes;
  longBreakMinutes = settings.pomodoro_long_break_minutes;
  cyclesBeforeLong = settings.pomodoro_cycles;
  if (!enabled) stopPomodoro();
}

function tick() {
  if (remainingSeconds <= 0) {
    onPhaseEnd();
    return;
  }
  remainingSeconds--;
}

function onPhaseEnd() {
  if (phase === "work") {
    cycleCount++;
    if (cycleCount % cyclesBeforeLong === 0) {
      startPhase("long-break", longBreakMinutes * 60);
      sendNotification({ title: t("app.name"), body: t("pomodoro.long_break_start") });
    } else {
      startPhase("break", breakMinutes * 60);
      sendNotification({ title: t("app.name"), body: t("pomodoro.break_start") });
    }
  } else {
    startPhase("work", workMinutes * 60);
    sendNotification({ title: t("app.name"), body: t("pomodoro.work_start") });
  }
}

function startPhase(newPhase: PomodoroPhase, seconds: number) {
  phase = newPhase;
  remainingSeconds = seconds;
  if (timerHandle) clearInterval(timerHandle);
  timerHandle = setInterval(tick, 1000);
}

export function startPomodoro() {
  if (!enabled) return;
  cycleCount = 0;
  startPhase("work", workMinutes * 60);
}

export function stopPomodoro() {
  if (timerHandle) {
    clearInterval(timerHandle);
    timerHandle = null;
  }
  phase = "idle";
  remainingSeconds = 0;
  cycleCount = 0;
}

export function skipPhase() {
  remainingSeconds = 0;
  onPhaseEnd();
}

export function formatPomodoroTime(seconds: number): string {
  const m = Math.floor(seconds / 60);
  const s = seconds % 60;
  return `${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
}
