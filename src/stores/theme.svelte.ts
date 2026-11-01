export type ThemeSetting = "dark" | "light" | "auto";
type Theme = "dark" | "light";

const raw = localStorage.getItem("bitclock-theme");
let setting: ThemeSetting = raw === "light" || raw === "dark" || raw === "auto" ? raw : "dark";

function resolveTheme(): Theme {
  if (setting === "auto") {
    const hour = new Date().getHours();
    return hour >= 7 && hour < 19 ? "light" : "dark";
  }
  return setting;
}

let current: Theme = resolveTheme();

export function getTheme(): Theme {
  return current;
}

export function getThemeSetting(): ThemeSetting {
  return setting;
}

export function toggleTheme() {
  setting = current === "dark" ? "light" : "dark";
  current = setting as Theme;
  localStorage.setItem("bitclock-theme", setting);
}

export function setTheme(value: ThemeSetting) {
  setting = value;
  current = resolveTheme();
  localStorage.setItem("bitclock-theme", value);
}

export function checkAutoTheme() {
  if (setting === "auto") {
    current = resolveTheme();
  }
}
