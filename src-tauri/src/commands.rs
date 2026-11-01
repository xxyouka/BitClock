use crate::db::{AppUsage, AppDetail, ComparisonData, CurrentApp, DailySummary, ExportRecord, HourlyUsage, Settings, TodayStats};
use crate::error::AppError;
use log::{error, info, warn};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::State;

use crate::db::Database;

#[tauri::command]
pub fn log_message(level: String, message: String) {
    match level.as_str() {
        "error" => error!("[frontend] {}", message),
        "warn" => warn!("[frontend] {}", message),
        "info" => info!("[frontend] {}", message),
        _ => info!("[frontend] {}", message),
    }
}

#[tauri::command]
pub fn backup_database(dest_path: String) -> Result<usize, AppError> {
    let src = Database::get_db_path();
    let dest = std::path::Path::new(&dest_path);
    // Copy main DB first for a consistent snapshot, then WAL/SHM
    std::fs::copy(&src, dest).map_err(|e| AppError::io(e))?;
    let wal = src.with_extension("db-wal");
    let shm = src.with_extension("db-shm");
    let dest_wal = std::path::PathBuf::from(format!("{}-wal", dest_path));
    let dest_shm = std::path::PathBuf::from(format!("{}-shm", dest_path));
    let count = 1
        + if wal.exists() { std::fs::copy(&wal, &dest_wal).map(|_| 1).unwrap_or(0) } else { 0 }
        + if shm.exists() { std::fs::copy(&shm, &dest_shm).map(|_| 1).unwrap_or(0) } else { 0 };
    Ok(count)
}

#[tauri::command]
pub fn restore_database(src_path: String) -> Result<(), AppError> {
    let dest = Database::get_db_path();
    std::fs::copy(&src_path, &dest).map_err(|e| AppError::io(e))?;
    Ok(())
}

#[tauri::command]
pub fn get_today_stats(
    db: State<'_, Arc<Mutex<Database>>>,
    current_app: State<'_, Arc<Mutex<Option<CurrentApp>>>>,
) -> Result<TodayStats, AppError> {
    let app_snapshot = current_app.lock().map_err(|e| AppError::lock(e))?.clone();
    let db = db.lock().map_err(|e| AppError::lock(e))?;
    db.get_today_stats(app_snapshot.as_ref()).map_err(AppError::from)
}

#[tauri::command]
pub fn get_app_ranking_range(
    db: State<'_, Arc<Mutex<Database>>>,
    start_date: String,
    end_date: String,
) -> Result<Vec<AppUsage>, AppError> {
    let db = db.lock().map_err(|e| AppError::lock(e))?;
    db.get_app_ranking_range(&start_date, &end_date).map_err(AppError::from)
}

#[tauri::command]
pub fn get_weekly_stats(db: State<'_, Arc<Mutex<Database>>>) -> Result<Vec<DailySummary>, AppError> {
    let db = db.lock().map_err(|e| AppError::lock(e))?;
    db.get_weekly_stats().map_err(AppError::from)
}

#[tauri::command]
pub fn get_monthly_stats(
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<Vec<DailySummary>, AppError> {
    let mut db = db.lock().map_err(|e| AppError::lock(e))?;
    db.get_monthly_stats().map_err(AppError::from)
}

#[tauri::command]
pub fn get_settings(db: State<'_, Arc<Mutex<Database>>>) -> Result<Settings, AppError> {
    let db = db.lock().map_err(|e| AppError::lock(e))?;
    db.get_settings().map_err(AppError::from)
}

#[tauri::command]
pub fn update_settings(
    db: State<'_, Arc<Mutex<Database>>>,
    new_settings: Settings,
) -> Result<(), AppError> {
    let db = db.lock().map_err(|e| AppError::lock(e))?;
    db.update_settings_atomic(&new_settings).map_err(AppError::from)
}

#[tauri::command]
pub fn toggle_autostart(enable: bool) -> Result<(), AppError> {
    #[cfg(target_os = "windows")]
    {
        use winapi::um::winnt::KEY_SET_VALUE;
        use winapi::um::winreg::{RegCloseKey, RegOpenKeyExW, RegSetValueExW, HKEY_CURRENT_USER};
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;

        let subkey = OsStr::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
        let subkey_w: Vec<u16> = subkey.encode_wide().chain(Some(0)).collect();

        let app_name = OsStr::new("BitClock");
        let app_name_w: Vec<u16> = app_name.encode_wide().chain(Some(0)).collect();

        let exe_path = std::env::current_exe().map_err(|e| AppError::io(e))?;
        let exe_path_str = exe_path.to_string_lossy().to_string();
        let exe_path_w: Vec<u16> = OsStr::new(&exe_path_str).encode_wide().chain(Some(0)).collect();

        unsafe {
            let mut hkey: winapi::shared::minwindef::HKEY = std::ptr::null_mut();
            let result = RegOpenKeyExW(
                HKEY_CURRENT_USER,
                subkey_w.as_ptr(),
                0,
                KEY_SET_VALUE,
                &mut hkey,
            );

            if result != 0 {
                return Err(AppError::registry(format!("Failed to open registry key: {}", result)));
            }

            if enable {
                let reg_result = RegSetValueExW(
                    hkey,
                    app_name_w.as_ptr(),
                    0,
                    winapi::um::winnt::REG_SZ,
                    exe_path_w.as_ptr() as *const u8,
                    (exe_path_w.len() * 2) as u32,
                );
                RegCloseKey(hkey);
                if reg_result != 0 {
                    return Err(AppError::registry(format!("Failed to set registry value: {}", reg_result)));
                }
            } else {
                let del_result = winapi::um::winreg::RegDeleteValueW(
                    hkey,
                    app_name_w.as_ptr(),
                );
                RegCloseKey(hkey);
                if del_result != 0 && del_result != 2 {
                    return Err(AppError::registry(format!("Failed to delete registry value: {}", del_result)));
                }
            }
        }
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = enable;
        Ok(())
    }
}

#[tauri::command]
pub fn get_current_app(
    current_app: State<'_, Arc<Mutex<Option<CurrentApp>>>>,
) -> Result<Option<CurrentApp>, AppError> {
    let state = current_app.lock().map_err(|e| AppError::lock(e))?;
    Ok(state.clone())
}

#[tauri::command]
pub fn cleanup_old_records(db: State<'_, Arc<Mutex<Database>>>) -> Result<usize, AppError> {
    let db = db.lock().map_err(|e| AppError::lock(e))?;
    let days = db.get_retention_days();
    db.cleanup_old_records(days).map_err(AppError::from)
}

#[tauri::command]
pub fn toggle_pause(paused: State<'_, Arc<Mutex<bool>>>) -> Result<bool, AppError> {
    let mut p = paused.lock().map_err(|e| AppError::lock(e))?;
    *p = !*p;
    Ok(*p)
}

#[tauri::command]
pub fn quit_app(
    app: tauri::AppHandle,
    running: State<'_, Arc<AtomicBool>>,
    tracker_handle: State<'_, Arc<Mutex<Option<std::thread::JoinHandle<()>>>>>,
) {
    running.store(false, Ordering::Release);
    if let Ok(mut handle) = tracker_handle.lock() {
        if let Some(h) = handle.take() {
            let _ = h.join();
        }
    }
    app.exit(0);
}

#[tauri::command]
pub fn export_data(db: State<'_, Arc<Mutex<Database>>>) -> Result<Vec<ExportRecord>, AppError> {
    let db = db.lock().map_err(|e| AppError::lock(e))?;
    db.export_all_records().map_err(AppError::from)
}

#[tauri::command]
pub fn export_to_file(db: State<'_, Arc<Mutex<Database>>>, path: String) -> Result<usize, AppError> {
    let db = db.lock().map_err(|e| AppError::lock(e))?;
    let count = db.export_to_file(std::path::Path::new(&path)).map_err(|e| AppError::io(e))?;
    Ok(count)
}

#[tauri::command]
pub fn export_csv_to_file(db: State<'_, Arc<Mutex<Database>>>, path: String) -> Result<usize, AppError> {
    let db = db.lock().map_err(|e| AppError::lock(e))?;
    let count = db.export_csv_to_file(std::path::Path::new(&path)).map_err(|e| AppError::io(e))?;
    Ok(count)
}

#[tauri::command]
pub fn toggle_always_on_top(app: tauri::AppHandle) -> Result<bool, AppError> {
    if let Some(window) = app.get_webview_window("main") {
        let is_top = window.is_always_on_top().unwrap_or(false);
        window.set_always_on_top(!is_top).map_err(|e| AppError::io(e))?;
        Ok(!is_top)
    } else {
        Err(AppError::io("Window not found"))
    }
}

#[tauri::command]
pub fn get_hourly_stats(db: State<'_, Arc<Mutex<Database>>>) -> Result<Vec<HourlyUsage>, AppError> {
    let db = db.lock().map_err(|e| AppError::lock(e))?;
    db.get_hourly_stats().map_err(AppError::from)
}

#[tauri::command]
pub fn get_app_detail(
    db: State<'_, Arc<Mutex<Database>>>,
    process_name: String,
    days: Option<i64>,
) -> Result<AppDetail, AppError> {
    let db = db.lock().map_err(|e| AppError::lock(e))?;
    db.get_app_detail(&process_name, days.unwrap_or(30)).map_err(AppError::from)
}

#[tauri::command]
pub fn get_comparison(
    db: State<'_, Arc<Mutex<Database>>>,
    period: String,
) -> Result<ComparisonData, AppError> {
    let db = db.lock().map_err(|e| AppError::lock(e))?;
    db.get_comparison(&period).map_err(AppError::from)
}

#[tauri::command]
pub fn set_locale(
    db: State<'_, Arc<Mutex<Database>>>,
    locale: String,
) -> Result<(), AppError> {
    let valid_locale = if locale == "en" { "en" } else { "zh" };
    let mut db = db.lock().map_err(|e| AppError::lock(e))?;
    db.update_setting("locale", valid_locale).map_err(AppError::from)
}

#[tauri::command]
pub fn set_app_category(
    db: State<'_, Arc<Mutex<Database>>>,
    process_name: String,
    category: String,
) -> Result<(), AppError> {
    let mut db = db.lock().map_err(|e| AppError::lock(e))?;
    db.set_app_category(&process_name, &category).map_err(AppError::from)
}

#[tauri::command]
pub fn get_category_stats(
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<Vec<(String, i64)>, AppError> {
    let db = db.lock().map_err(|e| AppError::lock(e))?;
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    db.get_category_stats(&today).map_err(AppError::from)
}

#[tauri::command]
pub fn get_all_categories(
    db: State<'_, Arc<Mutex<Database>>>,
) -> Result<Vec<(String, String)>, AppError> {
    let db = db.lock().map_err(|e| AppError::lock(e))?;
    db.get_all_categories().map_err(AppError::from)
}
