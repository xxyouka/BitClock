import { formatHMS } from "../utils/format";

// TODO: These types mirror the Rust structs with ts-rs derives.
// After running `cargo test` in src-tauri to generate individual type files,
// replace these with re-exports from "../types/generated".
export interface AppUsage {
  process_name: string;
  window_title: string;
  duration_sec: number;
  percentage: number;
}

export interface DailySummary {
  date: string;
  total_sec: number;
  app_count: number;
}

export interface CurrentApp {
  process_name: string;
  window_title: string;
  duration_sec: number;
}

export interface ExportRecord {
  process_name: string;
  window_title: string;
  start_time: string;
  end_time: string;
  duration_sec: number;
  date: string;
}

export interface TodayStats {
  total_sec: number;
  app_count: number;
  first_active: string | null;
  last_active: string | null;
  current_app: CurrentApp | null;
  app_ranking: AppUsage[];
}

const emptyStats: TodayStats = {
  total_sec: 0,
  app_count: 0,
  first_active: null,
  last_active: null,
  current_app: null,
  app_ranking: [],
};

let stats = $state.raw<TodayStats>(emptyStats);

export function getTodayStats(): TodayStats {
  return stats;
}

export function setTodayStats(value: TodayStats) {
  stats = value;
}

export function getFormattedTotalTime(): string {
  return formatHMS(stats.total_sec);
}
