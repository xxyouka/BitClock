mod commands;
mod db;
mod error;
mod tracker;
mod tray;

use log::error;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use tauri::Manager;

fn setup_logging() {
    let log_dir = db::Database::get_db_path()
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    let log_path = log_dir.join("bitclock.log");

    let log_file = match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open log file {:?}: {}", log_path, e);
            return;
        }
    };

    use std::io::Write;
    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .filter_level(log::LevelFilter::Info)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();

    // Log panics
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let thread = std::thread::current();
        let thread_name = thread.name().unwrap_or("unnamed");

        let payload = if let Some(s) = info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "unknown panic".to_string()
        };

        let location = info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown location".to_string());

        error!(
            "PANIC on thread '{}' at {}: {}",
            thread_name, location, payload
        );

        default_hook(info);
    }));
}

pub fn run() {
    setup_logging();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // Initialize database
            let db = match db::Database::new() {
                Ok(db) => db,
                Err(e) => {
                    eprintln!("Failed to initialize database: {}", e);
                    return Err(Box::new(e));
                }
            };

            // Cleanup old records and reclaim space on startup
            let retention = db.get_retention_days();
            if let Ok(deleted) = db.cleanup_old_records(retention) {
                if deleted > 0 {
                    println!("Cleaned up {} old records", deleted);
                }
            }
            let _ = db.incremental_vacuum();

            let db = Arc::new(Mutex::new(db));

            // Shared state
            let current_app_state: Arc<Mutex<Option<db::CurrentApp>>> = Arc::new(Mutex::new(None));
            let paused: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
            let running = Arc::new(AtomicBool::new(true));
            let tracker_handle: Arc<Mutex<Option<std::thread::JoinHandle<()>>>> =
                Arc::new(Mutex::new(None));

            // Store in app state
            app.manage(db.clone());
            app.manage(current_app_state.clone());
            app.manage(paused.clone());
            app.manage(running.clone());
            app.manage(tracker_handle.clone());

            // Start tracker in background
            let db_clone = db.clone();
            let app_state_clone = current_app_state.clone();
            let paused_clone = paused.clone();
            let running_clone = running.clone();
            let app_handle = app.handle().clone();
            let handle = std::thread::spawn(move || {
                let mut tracker = tracker::WindowTracker::new(
                    db_clone,
                    app_state_clone,
                    paused_clone,
                    running_clone,
                    Some(app_handle),
                );
                tracker.start();
            });

            if let Ok(mut h) = tracker_handle.lock() {
                *h = Some(handle);
            }

            // Read locale from settings for tray menu language
            let locale = {
                let mut db_guard = db.lock().unwrap();
                db_guard.get_setting_value("locale").unwrap_or_else(|| "zh".to_string())
            };
            tray::setup_tray(app, &locale)?;

            // Intercept window close: hide to tray instead of closing
            if let Some(window) = app.get_webview_window("main") {
                let app_handle = app.handle().clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        if let Some(w) = app_handle.get_webview_window("main") {
                            w.hide().ok();
                        }
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_today_stats,
            commands::get_app_ranking_range,
            commands::get_weekly_stats,
            commands::get_monthly_stats,
            commands::get_settings,
            commands::update_settings,
            commands::toggle_autostart,
            commands::get_current_app,
            commands::cleanup_old_records,
            commands::toggle_pause,
            commands::quit_app,
            commands::export_data,
            commands::export_to_file,
            commands::export_csv_to_file,
            commands::backup_database,
            commands::restore_database,
            commands::log_message,
            commands::toggle_always_on_top,
            commands::get_hourly_stats,
            commands::get_app_detail,
            commands::get_comparison,
            commands::set_locale,
            commands::set_app_category,
            commands::get_category_stats,
            commands::get_all_categories,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
