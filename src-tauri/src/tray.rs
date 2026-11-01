use tauri::{
    App, Manager, Emitter,
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState},
    menu::{Menu, MenuItem},
};

fn format_tray_time(seconds: i64) -> String {
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    if h > 0 {
        format!("{}h{}m", h, m)
    } else if m > 0 {
        format!("{}m", m)
    } else {
        "<1m".to_string()
    }
}

fn tray_text(locale: &str, zh: &str, en: &str) -> String {
    if locale == "en" { en.to_string() } else { zh.to_string() }
}

pub fn setup_tray(app: &App, locale: &str) -> Result<(), Box<dyn std::error::Error>> {
    let open_item = MenuItem::with_id(app, "open", tray_text(locale, "打开主界面", "Open"), true, None::<&str>)?;
    let pause_item = MenuItem::with_id(app, "pause", tray_text(locale, "暂停追踪", "Pause Tracking"), true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, "settings", tray_text(locale, "设置", "Settings"), true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", tray_text(locale, "退出 BitClock", "Quit BitClock"), true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&open_item, &pause_item, &settings_item, &quit_item])?;

    let pause_item_clone = pause_item.clone();
    let locale_clone = locale.to_string();

    // Listen for pause state changes from the frontend
    app.listen("pause-changed", move |event| {
        let is_paused = event.payload() == "true";
        let text = if is_paused {
            tray_text(&locale_clone, "继续追踪", "Resume Tracking")
        } else {
            tray_text(&locale_clone, "暂停追踪", "Pause Tracking")
        };
        pause_item_clone.set_text(&text).ok();
    });

    let tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().expect("default window icon not configured").clone())
        .tooltip("BitClock")
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "open" => {
                if let Some(window) = app.get_webview_window("main") {
                    window.show().ok();
                    window.set_focus().ok();
                    app.emit("navigate", "home").ok();
                }
            }
            "pause" => {
                app.emit("tray-pause", ()).ok();
            }
            "settings" => {
                if let Some(window) = app.get_webview_window("main") {
                    window.show().ok();
                    window.set_focus().ok();
                    app.emit("navigate", "settings").ok();
                }
            }
            "quit" => {
                app.emit("tray-quit", ()).ok();
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    window.show().ok();
                    window.set_focus().ok();
                }
            }
        })
        .build(app)?;

    // Update tray tooltip with today's stats
    let tray_clone = tray.clone();
    let locale_for_tooltip = locale.to_string();
    app.listen("stats-updated", move |event| {
        if let Ok(total_sec) = event.payload().parse::<i64>() {
            let text = if locale_for_tooltip == "en" {
                format!("BitClock - Today {}", format_tray_time(total_sec))
            } else {
                format!("BitClock 比特时钟 - 今日 {}", format_tray_time(total_sec))
            };
            let _ = tray_clone.set_tooltip(Some(&text));
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_tray_time_minutes_only() {
        assert_eq!(format_tray_time(0), "<1m");
        assert_eq!(format_tray_time(59), "<1m");
        assert_eq!(format_tray_time(60), "1m");
        assert_eq!(format_tray_time(300), "5m");
        assert_eq!(format_tray_time(3599), "59m");
    }

    #[test]
    fn test_tray_text() {
        assert_eq!(tray_text("zh", "打开", "Open"), "打开");
        assert_eq!(tray_text("en", "打开", "Open"), "Open");
        assert_eq!(tray_text("other", "打开", "Open"), "打开");
    }

    #[test]
    fn test_format_tray_time_hours() {
        assert_eq!(format_tray_time(3600), "1h0m");
        assert_eq!(format_tray_time(3660), "1h1m");
        assert_eq!(format_tray_time(7200), "2h0m");
        assert_eq!(format_tray_time(5400), "1h30m");
        assert_eq!(format_tray_time(28800), "8h0m");
    }
}
