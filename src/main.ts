import { mount } from "svelte";
import { invoke } from "@tauri-apps/api/core";
import "./styles/global.css";
import App from "./App.svelte";

const target = document.getElementById("app");
if (!target) {
  throw new Error("Missing #app element in index.html");
}

function logToBackend(level: string, message: string) {
  invoke("log_message", { level, message }).catch(() => {
    // Fallback if backend is unavailable
    console.error(`[${level}] ${message}`);
  });
}

// Global error handler for unhandled errors
window.addEventListener("error", (event) => {
  const msg = event.error ? String(event.error) : event.message;
  console.error("Unhandled error:", msg);
  logToBackend("error", `Unhandled: ${msg}`);
});
window.addEventListener("unhandledrejection", (event) => {
  const msg = String(event.reason);
  console.error("Unhandled rejection:", msg);
  logToBackend("error", `Unhandled rejection: ${msg}`);
});

const app = mount(App, { target });

export default app;
