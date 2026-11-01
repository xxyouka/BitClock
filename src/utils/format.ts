import { currentLocale } from "../stores/i18n.svelte";

/** Format seconds to HH:MM:SS */
export function formatHMS(seconds: number): string {
  const s = Math.max(0, Math.floor(seconds));
  const hours = Math.floor(s / 3600);
  const minutes = Math.floor((s % 3600) / 60);
  const secs = s % 60;
  return `${pad(hours)}:${pad(minutes)}:${pad(secs)}`;
}

/** Format seconds to compact "Xh Xm" or "Xs" format */
export function formatCompact(seconds: number): string {
  const s = Math.max(0, Math.floor(seconds));
  const hours = Math.floor(s / 3600);
  const minutes = Math.floor((s % 3600) / 60);
  if (hours > 0) {
    return `${hours}h${minutes}m`;
  }
  if (minutes > 0) {
    return `${minutes}m`;
  }
  return `${s}s`;
}

function pad(n: number): string {
  return String(Math.max(0, n)).padStart(2, "0");
}

const DAY_NAMES_ZH = ["日", "一", "二", "三", "四", "五", "六"];
const DAY_NAMES_EN = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

export function getDayLabel(dateStr: string): string {
  const [y, m, d] = dateStr.split("-").map(Number);
  const date = new Date(y, m - 1, d);
  // Read currentLocale to establish reactive dependency — called during render
  return currentLocale === "en" ? DAY_NAMES_EN[date.getDay()] : DAY_NAMES_ZH[date.getDay()];
}

export function formatDateShort(dateStr: string): string {
  const parts = dateStr.split("-");
  if (parts.length < 3) return dateStr;
  return `${parts[1]}-${parts[2]}`;
}
