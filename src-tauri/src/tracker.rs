use chrono::Local;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

use crate::db::{CurrentApp, Database};

#[cfg(target_os = "windows")]
type WindowHandle = winapi::shared::windef::HWND;

pub struct WindowTracker {
    db: Arc<Mutex<Database>>,
    current_process: Option<String>,
    current_title: Option<String>,
    start_time: Option<chrono::NaiveDateTime>,
    current_row_id: Option<i64>,
    current_app_state: Arc<Mutex<Option<CurrentApp>>>,
    checkpoint_counter: u32,
    paused: Arc<Mutex<bool>>,
    running: Arc<AtomicBool>,
    app_handle: Option<AppHandle>,
    #[cfg(target_os = "windows")]
    last_hwnd: Option<WindowHandle>,
    idle_threshold_secs: u64,
    is_idle: bool,
    settings_refresh_counter: u32,
}

impl WindowTracker {
    pub fn new(
        db: Arc<Mutex<Database>>,
        current_app_state: Arc<Mutex<Option<CurrentApp>>>,
        paused: Arc<Mutex<bool>>,
        running: Arc<AtomicBool>,
        app_handle: Option<AppHandle>,
    ) -> Self {
        let idle_threshold_secs = Self::read_idle_threshold(&db);
        WindowTracker {
            db,
            current_process: None,
            current_title: None,
            start_time: None,
            current_row_id: None,
            current_app_state,
            checkpoint_counter: 0,
            paused,
            running,
            app_handle,
            #[cfg(target_os = "windows")]
            last_hwnd: None,
            idle_threshold_secs,
            is_idle: false,
            settings_refresh_counter: 0,
        }
    }

    fn read_idle_threshold(db: &Arc<Mutex<Database>>) -> u64 {
        match db.lock() {
            Ok(mut db) => {
                let minutes = db.get_settings()
                    .map(|s| s.idle_threshold_minutes)
                    .unwrap_or(5);
                (minutes.max(1) * 60) as u64
            }
            Err(_) => 300,
        }
    }

    fn emit_app_changed(&self, app: Option<&CurrentApp>) {
        if let Some(handle) = &self.app_handle {
            let _ = handle.emit("foreground-changed", app.cloned());
        }
    }

    #[cfg(target_os = "windows")]
    fn get_idle_seconds(&self) -> u64 {
        use winapi::um::winuser::GetLastInputInfo;
        use winapi::um::winuser::LASTINPUTINFO;

        unsafe {
            let mut lii = LASTINPUTINFO {
                cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
                dwTime: 0,
            };
            if GetLastInputInfo(&mut lii) != 0 {
                let tick = winapi::um::sysinfoapi::GetTickCount();
                let elapsed_ms = tick.wrapping_sub(lii.dwTime);
                (elapsed_ms / 1000) as u64
            } else {
                0
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    fn get_idle_seconds(&self) -> u64 {
        0
    }

    pub fn start(&mut self) {
        while self.running.load(Ordering::Acquire) {
            let is_paused = match self.paused.lock() {
                Ok(guard) => *guard,
                Err(poisoned) => {
                    eprintln!("Paused mutex poisoned, recovering");
                    *poisoned.into_inner()
                }
            };

            if is_paused {
                self.finalize();
                self.is_idle = false;
                self.checkpoint_counter = 0;
                match self.current_app_state.lock() {
                    Ok(mut state) => *state = None,
                    Err(poisoned) => {
                        eprintln!("App state mutex poisoned, recovering");
                        *poisoned.into_inner() = None;
                    }
                }
                self.emit_app_changed(None);
                thread::sleep(Duration::from_secs(1));
                continue;
            }

            // Idle detection
            let idle_secs = self.get_idle_seconds();
            if idle_secs >= self.idle_threshold_secs {
                if !self.is_idle {
                    self.is_idle = true;
                    self.finalize();
                    self.checkpoint_counter = 0;
                    match self.current_app_state.lock() {
                        Ok(mut state) => *state = None,
                        Err(poisoned) => {
                            eprintln!("App state mutex poisoned, recovering");
                            *poisoned.into_inner() = None;
                        }
                    }
                    self.emit_app_changed(None);
                }
                thread::sleep(Duration::from_secs(5));
                continue;
            } else if self.is_idle {
                self.is_idle = false;
            }

            let current_hwnd = self.get_foreground_hwnd();
            let hwnd_changed = current_hwnd != self.last_hwnd;

            if let Some(hwnd) = current_hwnd {
                let info = if hwnd_changed {
                    self.last_hwnd = Some(hwnd);
                    self.get_window_info(hwnd)
                } else {
                    None
                };

                let (process_name, window_title) = if let Some(info) = info {
                    info
                } else if let (Some(p), Some(t)) = (&self.current_process, &self.current_title) {
                    (p.clone(), t.clone())
                } else {
                    match self.get_window_info_full(hwnd) {
                        Some(info) => info,
                        None => continue,
                    }
                };

                let should_update = match (&self.current_process, &self.current_title) {
                    (Some(current_process), Some(current_title)) => {
                        current_process != &process_name || current_title != &window_title
                    }
                    _ => true,
                };

                if should_update {
                    self.finalize();

                    self.current_process = Some(process_name.clone());
                    self.current_title = Some(window_title.clone());
                    self.start_time = Some(Local::now().naive_local());
                    self.current_row_id = self.insert_new_record(&process_name, &window_title);
                    self.checkpoint_counter = 0;
                }

                if let (Some(process), Some(title), Some(start)) = (
                    &self.current_process,
                    &self.current_title,
                    &self.start_time,
                ) {
                    let now = Local::now().naive_local();
                    let duration = (now - *start).num_seconds();
                    let app = CurrentApp {
                        process_name: process.clone(),
                        window_title: title.clone(),
                        duration_sec: duration,
                    };
                    match self.current_app_state.lock() {
                        Ok(mut state) => *state = Some(app.clone()),
                        Err(poisoned) => {
                            eprintln!("App state mutex poisoned, recovering");
                            *poisoned.into_inner() = Some(app.clone());
                        }
                    }
                    if should_update {
                        self.emit_app_changed(Some(&app));
                    }
                }
            } else {
                self.last_hwnd = None;
                if self.current_process.is_some() {
                    self.finalize();
                    self.checkpoint_counter = 0;
                    match self.current_app_state.lock() {
                        Ok(mut state) => *state = None,
                        Err(poisoned) => {
                            eprintln!("App state mutex poisoned, recovering");
                            *poisoned.into_inner() = None;
                        }
                    }
                    self.emit_app_changed(None);
                }
            }

            self.checkpoint_counter += 1;
            if self.checkpoint_counter >= 15 {
                self.checkpoint();
                self.checkpoint_counter = 0;
            }

            self.settings_refresh_counter += 1;
            if self.settings_refresh_counter >= 60 {
                self.idle_threshold_secs = Self::read_idle_threshold(&self.db);
                self.settings_refresh_counter = 0;
            }

            let sleep_secs = if self.current_process.is_some() { 2 } else { 5 };
            thread::sleep(Duration::from_secs(sleep_secs));
        }

        self.finalize();
    }

    fn insert_new_record(&self, process_name: &str, window_title: &str) -> Option<i64> {
        let now = Local::now().naive_local();
        match self.db.lock() {
            Ok(db) => match db.insert_usage(process_name, window_title, now, now, 0) {
                Ok(id) => Some(id),
                Err(e) => {
                    eprintln!("Failed to insert usage record: {}", e);
                    None
                }
            },
            Err(e) => {
                eprintln!("Failed to acquire DB lock: {}", e);
                None
            }
        }
    }

    fn checkpoint(&self) {
        if let (Some(_process), Some(_title), Some(start), Some(row_id)) = (
            &self.current_process,
            &self.current_title,
            &self.start_time,
            self.current_row_id,
        ) {
            let now = Local::now().naive_local();
            let duration = (now - *start).num_seconds();
            if duration > 0 {
                if let Ok(db) = self.db.lock() {
                    if let Err(e) = db.update_record(row_id, now, duration) {
                        eprintln!("Failed to checkpoint record: {}", e);
                    }
                }
            }
        }
    }

    fn finalize(&mut self) {
        self.checkpoint();
        self.current_row_id = None;
        self.current_process = None;
        self.current_title = None;
        self.start_time = None;
    }

    #[cfg(target_os = "windows")]
    fn get_foreground_hwnd(&self) -> Option<WindowHandle> {
        use winapi::um::winuser::GetForegroundWindow;
        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd.is_null() { None } else { Some(hwnd) }
        }
    }

    #[cfg(target_os = "windows")]
    fn get_window_info(&self, hwnd: WindowHandle) -> Option<(String, String)> {
        self.get_window_info_full(hwnd)
    }

    #[cfg(target_os = "windows")]
    fn get_window_info_full(&self, hwnd: WindowHandle) -> Option<(String, String)> {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        use winapi::um::winuser::{GetWindowTextW, GetWindowThreadProcessId};
        use winapi::um::processthreadsapi::OpenProcess;
        use winapi::um::psapi::GetModuleBaseNameW;
        use winapi::um::handleapi::CloseHandle;
        use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

        unsafe {
            let mut title_buf = [0u16; 512];
            let title_len = GetWindowTextW(hwnd, title_buf.as_mut_ptr(), 512);
            let title = if title_len == 0 {
                "(Desktop)".to_string()
            } else {
                OsString::from_wide(&title_buf[..title_len as usize])
                    .to_string_lossy()
                    .to_string()
            };

            let mut process_id: u32 = 0;
            GetWindowThreadProcessId(hwnd, &mut process_id);

            let process_handle = OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                0,
                process_id,
            );

            if process_handle.is_null() {
                return None;
            }

            let mut name_buf = [0u16; 512];
            let name_len = GetModuleBaseNameW(
                process_handle,
                std::ptr::null_mut(),
                name_buf.as_mut_ptr(),
                512,
            );

            CloseHandle(process_handle);

            if name_len == 0 {
                return None;
            }

            let process_name = OsString::from_wide(&name_buf[..name_len as usize])
                .to_string_lossy()
                .to_string();

            Some((process_name, title))
        }
    }

    #[cfg(not(target_os = "windows"))]
    fn get_foreground_hwnd(&self) -> Option<()> {
        eprintln!("BitClock: foreground window tracking is only supported on Windows");
        None
    }

    #[cfg(not(target_os = "windows"))]
    fn get_window_info(&self, _: ()) -> Option<(String, String)> {
        None
    }

    #[cfg(not(target_os = "windows"))]
    fn get_window_info_full(&self, _: ()) -> Option<(String, String)> {
        None
    }
}
