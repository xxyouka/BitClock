export interface Settings {
  autostart: boolean;
  reminder_on_exit: boolean;
  daily_goal_minutes: number;
  retention_days: number;
  idle_threshold_minutes: number;
  break_reminder_minutes: number;
  custom_accent_color: string;
  pomodoro_enabled: boolean;
  pomodoro_work_minutes: number;
  pomodoro_break_minutes: number;
  pomodoro_long_break_minutes: number;
  pomodoro_cycles: number;
}

const defaults: Settings = {
  autostart: true,
  reminder_on_exit: true,
  daily_goal_minutes: 0,
  retention_days: 90,
  idle_threshold_minutes: 5,
  break_reminder_minutes: 45,
  custom_accent_color: "",
  pomodoro_enabled: false,
  pomodoro_work_minutes: 25,
  pomodoro_break_minutes: 5,
  pomodoro_long_break_minutes: 15,
  pomodoro_cycles: 4,
};

let current = $state.raw<Settings>(defaults);

export function getSettings(): Settings {
  return current;
}

export function setSettings(value: Settings) {
  current = value;
}
