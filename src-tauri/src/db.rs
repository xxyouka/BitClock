use chrono::{Local, NaiveDateTime};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
#[ts(export, export_to = "../types/generated/")]
pub struct AppUsage {
    pub process_name: String,
    pub window_title: String,
    pub duration_sec: i64,
    pub percentage: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
#[ts(export, export_to = "../types/generated/")]
pub struct DailySummary {
    pub date: String,
    pub total_sec: i64,
    pub app_count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
#[ts(export, export_to = "../types/generated/")]
pub struct TodayStats {
    pub total_sec: i64,
    pub app_count: i64,
    pub first_active: Option<String>,
    pub last_active: Option<String>,
    pub current_app: Option<CurrentApp>,
    pub app_ranking: Vec<AppUsage>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
#[ts(export, export_to = "../types/generated/")]
pub struct CurrentApp {
    pub process_name: String,
    pub window_title: String,
    pub duration_sec: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
#[ts(export, export_to = "../types/generated/")]
pub struct Settings {
    pub autostart: bool,
    pub reminder_on_exit: bool,
    pub daily_goal_minutes: i64,
    pub retention_days: i64,
    pub idle_threshold_minutes: i64,
    pub break_reminder_minutes: i64,
    pub custom_accent_color: String,
    pub pomodoro_enabled: bool,
    pub pomodoro_work_minutes: i64,
    pub pomodoro_break_minutes: i64,
    pub pomodoro_long_break_minutes: i64,
    pub pomodoro_cycles: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
#[ts(export, export_to = "../types/generated/")]
pub struct ExportRecord {
    pub process_name: String,
    pub window_title: String,
    pub start_time: String,
    pub end_time: String,
    pub duration_sec: i64,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
#[ts(export, export_to = "../types/generated/")]
pub struct HourlyUsage {
    pub hour: i64,
    pub duration_sec: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
#[ts(export, export_to = "../types/generated/")]
pub struct AppDetail {
    pub process_name: String,
    pub daily_trend: Vec<DailySummary>,
    pub window_titles: Vec<(String, i64)>,
    pub total_sec: i64,
    pub avg_daily_sec: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
#[ts(export, export_to = "../types/generated/")]
pub struct ComparisonData {
    pub current_total: i64,
    pub previous_total: i64,
    pub current_avg: f64,
    pub previous_avg: f64,
    pub current_days: i64,
    pub previous_days: i64,
    pub change_pct: f64,
}

struct QueryCache<T: Clone> {
    data: Option<T>,
    timestamp: Option<Instant>,
    ttl_secs: u64,
}

impl<T: Clone> QueryCache<T> {
    fn new(ttl_secs: u64) -> Self {
        QueryCache { data: None, timestamp: None, ttl_secs }
    }

    fn get(&self) -> Option<&T> {
        if let (Some(data), Some(ts)) = (&self.data, &self.timestamp) {
            if ts.elapsed().as_secs() < self.ttl_secs {
                return Some(data);
            }
        }
        None
    }

    fn set(&mut self, data: T) {
        self.data = Some(data);
        self.timestamp = Some(Instant::now());
    }

    fn invalidate(&mut self) {
        self.data = None;
        self.timestamp = None;
    }
}

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

pub struct Database {
    conn: Connection,
    today_cache: QueryCache<TodayStats>,
    weekly_cache: QueryCache<Vec<DailySummary>>,
    monthly_cache: QueryCache<Vec<DailySummary>>,
    settings_cache: QueryCache<Settings>,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path();
        let dir = db_path.parent().ok_or_else(|| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CANTOPEN),
                Some(format!("Invalid database path: {}", db_path.display())),
            )
        })?;
        std::fs::create_dir_all(dir).map_err(|e| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CANTOPEN),
                Some(format!("Failed to create directory {}: {}", dir.display(), e)),
            )
        })?;

        let conn = Connection::open(db_path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA auto_vacuum=INCREMENTAL;")?;

        let db = Database {
            conn,
            today_cache: QueryCache::new(5),
            weekly_cache: QueryCache::new(30),
            monthly_cache: QueryCache::new(60),
            settings_cache: QueryCache::new(60),
        };
        db.init_tables()?;
        Ok(db)
    }

    pub fn get_db_path() -> PathBuf {
        let mut path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from(std::env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".to_string())));
        path.push("BitClock");
        path.push("bitclock.db");
        path
    }

    fn init_tables(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS app_usage (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                process_name TEXT NOT NULL,
                window_title TEXT NOT NULL DEFAULT '',
                start_time DATETIME NOT NULL,
                end_time DATETIME,
                duration_sec INTEGER DEFAULT 0,
                date DATE NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_date_process ON app_usage(date, process_name);

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS app_categories (
                process_name TEXT PRIMARY KEY,
                category TEXT NOT NULL DEFAULT '其他'
            );

            INSERT OR IGNORE INTO settings (key, value) VALUES ('autostart', 'true');
            INSERT OR IGNORE INTO settings (key, value) VALUES ('reminder_on_exit', 'true');
            INSERT OR IGNORE INTO settings (key, value) VALUES ('daily_goal_minutes', '0');
            INSERT OR IGNORE INTO settings (key, value) VALUES ('retention_days', '90');
            INSERT OR IGNORE INTO settings (key, value) VALUES ('idle_threshold_minutes', '5');
            INSERT OR IGNORE INTO settings (key, value) VALUES ('break_reminder_minutes', '45');
            INSERT OR IGNORE INTO settings (key, value) VALUES ('custom_accent_color', '');
            INSERT OR IGNORE INTO settings (key, value) VALUES ('pomodoro_enabled', 'false');
            INSERT OR IGNORE INTO settings (key, value) VALUES ('pomodoro_work_minutes', '25');
            INSERT OR IGNORE INTO settings (key, value) VALUES ('pomodoro_break_minutes', '5');
            INSERT OR IGNORE INTO settings (key, value) VALUES ('pomodoro_long_break_minutes', '15');
            INSERT OR IGNORE INTO settings (key, value) VALUES ('pomodoro_cycles', '4');
            ",
        )?;
        Ok(())
    }

    pub fn insert_usage(
        &mut self,
        process_name: &str,
        window_title: &str,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
        duration_sec: i64,
    ) -> Result<i64> {
        let date = start_time.format("%Y-%m-%d").to_string();
        let start_str = start_time.format("%Y-%m-%d %H:%M:%S").to_string();
        let end_str = end_time.format("%Y-%m-%d %H:%M:%S").to_string();
        self.conn.execute(
            "INSERT INTO app_usage (process_name, window_title, start_time, end_time, duration_sec, date)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![process_name, window_title, start_str, end_str, duration_sec, date],
        )?;
        self.today_cache.invalidate();
        self.weekly_cache.invalidate();
        self.monthly_cache.invalidate();
        Ok(self.conn.last_insert_rowid())
    }

    pub fn update_record(&mut self, row_id: i64, end_time: NaiveDateTime, duration_sec: i64) -> Result<()> {
        let end_str = end_time.format("%Y-%m-%d %H:%M:%S").to_string();
        self.conn.execute(
            "UPDATE app_usage SET end_time = ?1, duration_sec = ?2 WHERE id = ?3",
            params![end_str, duration_sec, row_id],
        )?;
        self.today_cache.invalidate();
        self.weekly_cache.invalidate();
        self.monthly_cache.invalidate();
        Ok(())
    }

    pub fn get_today_stats(&mut self, current_app: Option<&CurrentApp>) -> Result<TodayStats> {
        // Return cached stats (refresh current_app separately)
        if let Some(cached) = self.today_cache.get() {
            let mut result = cached.clone();
            result.current_app = current_app.cloned();
            return Ok(result);
        }

        let today = Local::now().format("%Y-%m-%d").to_string();

        let (total_sec, app_count, first_active, last_active): (i64, i64, Option<String>, Option<String>) =
            self.conn.query_row(
                "SELECT COALESCE(SUM(duration_sec), 0), COUNT(DISTINCT process_name),
                        MIN(start_time), MAX(end_time)
                 FROM app_usage WHERE date = ?1",
                params![today],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )?;

        let app_ranking = self.get_app_ranking(&today, total_sec)?;

        let stats = TodayStats {
            total_sec,
            app_count,
            first_active,
            last_active,
            current_app: None,
            app_ranking,
        };
        self.today_cache.set(stats.clone());

        let mut result = stats;
        result.current_app = current_app.cloned();
        Ok(result)
    }

    fn get_app_ranking(&self, date: &str, total: i64) -> Result<Vec<AppUsage>> {
        self.get_app_ranking_range_inner(date, date, Some(total))
    }

    pub fn get_app_ranking_range(&self, start_date: &str, end_date: &str) -> Result<Vec<AppUsage>> {
        self.get_app_ranking_range_inner(start_date, end_date, None)
    }

    fn get_app_ranking_range_inner(&self, start_date: &str, end_date: &str, total_override: Option<i64>) -> Result<Vec<AppUsage>> {
        let total_sec = match total_override {
            Some(t) => t,
            None => self.conn.query_row(
                "SELECT COALESCE(SUM(duration_sec), 0) FROM app_usage WHERE date >= ?1 AND date <= ?2",
                params![start_date, end_date],
                |row| row.get(0),
            )?,
        };

        let mut stmt = self.conn.prepare(
            "SELECT process_name,
                    (SELECT window_title FROM app_usage AS u2
                     WHERE u2.process_name = u1.process_name AND u2.date >= ?1 AND u2.date <= ?2
                     ORDER BY u2.start_time DESC LIMIT 1) AS window_title,
                    SUM(duration_sec) as total_duration
             FROM app_usage AS u1
             WHERE date >= ?1 AND date <= ?2
             GROUP BY process_name
             ORDER BY total_duration DESC
             LIMIT 10",
        )?;

        let rows = stmt.query_map(params![start_date, end_date], |row| {
            let duration: i64 = row.get(2)?;
            let percentage = if total_sec > 0 {
                (duration as f64 / total_sec as f64) * 100.0
            } else {
                0.0
            };
            Ok(AppUsage {
                process_name: row.get(0)?,
                window_title: row.get(1)?,
                duration_sec: duration,
                percentage,
            })
        })?;

        rows.collect()
    }

    fn get_daily_stats(&self, days: i64) -> Result<Vec<DailySummary>> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        let start_date = (Local::now() - chrono::Duration::days(days)).format("%Y-%m-%d").to_string();

        let mut stmt = self.conn.prepare(
            "SELECT date, SUM(duration_sec) as total_sec, COUNT(DISTINCT process_name) as app_count
             FROM app_usage
             WHERE date >= ?1 AND date <= ?2
             GROUP BY date
             ORDER BY date DESC",
        )?;

        let rows = stmt.query_map(params![start_date, today], |row| {
            Ok(DailySummary {
                date: row.get(0)?,
                total_sec: row.get(1)?,
                app_count: row.get(2)?,
            })
        })?;

        rows.collect()
    }

    pub fn get_weekly_stats(&mut self) -> Result<Vec<DailySummary>> {
        if let Some(cached) = self.weekly_cache.get() {
            return Ok(cached.clone());
        }
        let result = self.get_daily_stats(7)?;
        self.weekly_cache.set(result.clone());
        Ok(result)
    }

    pub fn get_monthly_stats(&mut self) -> Result<Vec<DailySummary>> {
        if let Some(cached) = self.monthly_cache.get() {
            return Ok(cached.clone());
        }
        let now = Local::now();
        let start_date = now.format("%Y-%m-01").to_string();
        let today = now.format("%Y-%m-%d").to_string();

        let mut stmt = self.conn.prepare(
            "SELECT date, SUM(duration_sec) as total_sec, COUNT(DISTINCT process_name) as app_count
             FROM app_usage
             WHERE date >= ?1 AND date <= ?2
             GROUP BY date
             ORDER BY date DESC",
        )?;

        let rows = stmt.query_map(params![start_date, today], |row| {
            Ok(DailySummary {
                date: row.get(0)?,
                total_sec: row.get(1)?,
                app_count: row.get(2)?,
            })
        })?;

        let result: Vec<DailySummary> = rows.collect::<Result<Vec<_>>>()?;
        self.monthly_cache.set(result.clone());
        Ok(result)
    }

    pub fn get_settings(&mut self) -> Result<Settings> {
        if let Some(cached) = self.settings_cache.get() {
            return Ok(cached.clone());
        }

        let mut stmt = self.conn.prepare("SELECT key, value FROM settings")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        let mut autostart = true;
        let mut reminder_on_exit = true;
        let mut daily_goal_minutes: i64 = 0;
        let mut retention_days: i64 = 90;
        let mut idle_threshold_minutes: i64 = 5;
        let mut break_reminder_minutes: i64 = 45;
        let mut custom_accent_color = String::new();
        let mut pomodoro_enabled = false;
        let mut pomodoro_work_minutes: i64 = 25;
        let mut pomodoro_break_minutes: i64 = 5;
        let mut pomodoro_long_break_minutes: i64 = 15;
        let mut pomodoro_cycles: i64 = 4;
        for row in rows {
            let (key, value) = row?;
            match key.as_str() {
                "autostart" => autostart = value == "true",
                "reminder_on_exit" => reminder_on_exit = value == "true",
                "daily_goal_minutes" => daily_goal_minutes = value.parse().unwrap_or(0),
                "retention_days" => retention_days = value.parse().unwrap_or(90),
                "idle_threshold_minutes" => idle_threshold_minutes = value.parse().unwrap_or(5),
                "break_reminder_minutes" => break_reminder_minutes = value.parse().unwrap_or(45),
                "custom_accent_color" => custom_accent_color = value,
                "pomodoro_enabled" => pomodoro_enabled = value == "true",
                "pomodoro_work_minutes" => pomodoro_work_minutes = value.parse().unwrap_or(25),
                "pomodoro_break_minutes" => pomodoro_break_minutes = value.parse().unwrap_or(5),
                "pomodoro_long_break_minutes" => pomodoro_long_break_minutes = value.parse().unwrap_or(15),
                "pomodoro_cycles" => pomodoro_cycles = value.parse().unwrap_or(4),
                _ => {}
            }
        }

        let settings = Settings { autostart, reminder_on_exit, daily_goal_minutes, retention_days, idle_threshold_minutes, break_reminder_minutes, custom_accent_color, pomodoro_enabled, pomodoro_work_minutes, pomodoro_break_minutes, pomodoro_long_break_minutes, pomodoro_cycles };
        self.settings_cache.set(settings.clone());
        Ok(settings)
    }

    pub fn get_setting_value(&mut self, key: &str) -> Option<String> {
        self.conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            params![key],
            |row| row.get(0),
        ).ok()
    }

    pub fn update_setting(&mut self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        self.settings_cache.invalidate();
        Ok(())
    }

    pub fn update_settings_atomic(&mut self, settings: &Settings) -> Result<()> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params!["autostart", settings.autostart.to_string()],
        )?;
        tx.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params!["reminder_on_exit", settings.reminder_on_exit.to_string()],
        )?;
        tx.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params!["daily_goal_minutes", settings.daily_goal_minutes.to_string()],
        )?;
        tx.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params!["retention_days", settings.retention_days.to_string()],
        )?;
        tx.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params!["idle_threshold_minutes", settings.idle_threshold_minutes.to_string()],
        )?;
        tx.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params!["break_reminder_minutes", settings.break_reminder_minutes.to_string()],
        )?;
        tx.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params!["custom_accent_color", settings.custom_accent_color],
        )?;
        tx.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params!["pomodoro_enabled", settings.pomodoro_enabled.to_string()],
        )?;
        tx.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params!["pomodoro_work_minutes", settings.pomodoro_work_minutes.to_string()],
        )?;
        tx.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params!["pomodoro_break_minutes", settings.pomodoro_break_minutes.to_string()],
        )?;
        tx.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params!["pomodoro_long_break_minutes", settings.pomodoro_long_break_minutes.to_string()],
        )?;
        tx.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params!["pomodoro_cycles", settings.pomodoro_cycles.to_string()],
        )?;
        tx.commit()?;
        self.settings_cache.invalidate();
        Ok(())
    }

    pub fn vacuum(&self) -> Result<()> {
        self.conn.execute_batch("VACUUM")?;
        Ok(())
    }

    pub fn incremental_vacuum(&self) -> Result<()> {
        self.conn.execute_batch("PRAGMA incremental_vacuum;")?;
        Ok(())
    }

    pub fn get_retention_days(&mut self) -> i64 {
        self.get_settings()
            .map(|s| s.retention_days)
            .unwrap_or(90)
    }

    pub fn export_all_records(&self) -> Result<Vec<ExportRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT process_name, window_title, start_time, end_time, duration_sec, date
             FROM app_usage ORDER BY start_time DESC LIMIT 100000",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(ExportRecord {
                process_name: row.get(0)?,
                window_title: row.get(1)?,
                start_time: row.get(2)?,
                end_time: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
                duration_sec: row.get(4)?,
                date: row.get(5)?,
            })
        })?;

        rows.collect()
    }

    pub fn export_to_file(&self, path: &std::path::Path) -> std::result::Result<usize, std::io::Error> {
        let file = std::fs::File::create(path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(b"[\n")?;

        let mut stmt = self.conn.prepare(
            "SELECT process_name, window_title, start_time, end_time, duration_sec, date
             FROM app_usage ORDER BY start_time DESC",
        ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let mut count: usize = 0;
        let mut rows = stmt.query_map([], |row| {
            Ok(ExportRecord {
                process_name: row.get(0)?,
                window_title: row.get(1)?,
                start_time: row.get(2)?,
                end_time: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
                duration_sec: row.get(4)?,
                date: row.get(5)?,
            })
        }).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let mut first = true;
        while let Some(row) = rows.next() {
            let record = row.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            if !first {
                writer.write_all(b",\n")?;
            }
            serde_json::to_writer(&mut writer, &record)?;
            first = false;
            count += 1;
        }

        writer.write_all(b"\n]")?;
        writer.flush()?;
        Ok(count)
    }

    pub fn export_csv_to_file(&self, path: &std::path::Path) -> std::result::Result<usize, std::io::Error> {
        let file = std::fs::File::create(path)?;
        let mut writer = BufWriter::new(file);
        // BOM for Excel compatibility
        writer.write_all(b"\xEF\xBB\xBF")?;
        writer.write_all("Date,App,Window Title,Start Time,End Time,Duration(sec)\n".as_bytes())?;

        let mut stmt = self.conn.prepare(
            "SELECT date, process_name, window_title, start_time, end_time, duration_sec
             FROM app_usage ORDER BY start_time DESC",
        ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let mut count: usize = 0;
        let mut rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?.unwrap_or_default(),
                row.get::<_, i64>(5)?,
            ))
        }).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        while let Some(row) = rows.next() {
            let (date, process, title, start, end, duration) =
                row.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            let line = format!(
                "{},{},{},{},{},{}\n",
                csv_escape(&date),
                csv_escape(&process),
                csv_escape(&title),
                csv_escape(&start),
                csv_escape(&end),
                duration,
            );
            writer.write_all(line.as_bytes())?;
            count += 1;
        }

        writer.flush()?;
        Ok(count)
    }

    pub fn set_app_category(&mut self, process_name: &str, category: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO app_categories (process_name, category) VALUES (?1, ?2)",
            params![process_name, category],
        )?;
        Ok(())
    }

    pub fn get_app_category(&self, process_name: &str) -> Result<String> {
        self.conn.query_row(
            "SELECT category FROM app_categories WHERE process_name = ?1",
            params![process_name],
            |row| row.get(0),
        ).unwrap_or_else(|_| "其他".to_string())
    }

    pub fn get_category_stats(&self, date: &str) -> Result<Vec<(String, i64)>> {
        let mut stmt = self.conn.prepare(
            "SELECT COALESCE(ac.category, '其他') as category, SUM(au.duration_sec) as total_duration
             FROM app_usage au
             LEFT JOIN app_categories ac ON au.process_name = ac.process_name
             WHERE au.date = ?1
             GROUP BY category
             ORDER BY total_duration DESC",
        )?;

        let rows = stmt.query_map(params![date], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?;

        rows.collect()
    }

    pub fn get_all_categories(&self) -> Result<Vec<(String, String)>> {
        let mut stmt = self.conn.prepare("SELECT process_name, category FROM app_categories")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        rows.collect()
    }

    pub fn get_hourly_stats(&self) -> Result<Vec<HourlyUsage>> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        let mut stmt = self.conn.prepare(
            "SELECT
                CAST(strftime('%H', start_time) AS INTEGER) as hour,
                SUM(
                    CASE
                        WHEN end_time IS NOT NULL THEN
                            MIN(CAST(strftime('%s', end_time) AS INTEGER), CAST(strftime('%s', date || ' 23:59:59') AS INTEGER))
                            - MAX(CAST(strftime('%s', start_time) AS INTEGER), CAST(strftime('%s', date || ' 00:00:00') AS INTEGER))
                        ELSE 0
                    END
                ) as total_sec
             FROM app_usage
             WHERE date = ?1
             GROUP BY hour
             ORDER BY hour",
        )?;

        let rows = stmt.query_map(params![today], |row| {
            Ok(HourlyUsage {
                hour: row.get(0)?,
                duration_sec: row.get::<_, i64>(1)?.max(0),
            })
        })?;

        let mut result: Vec<HourlyUsage> = rows.collect::<Result<Vec<_>>>()?;
        // Fill missing hours with 0
        let existing_hours: std::collections::HashSet<i64> = result.iter().map(|h| h.hour).collect();
        for hour in 0..24 {
            if !existing_hours.contains(&hour) {
                result.push(HourlyUsage { hour, duration_sec: 0 });
            }
        }
        result.sort_by_key(|h| h.hour);
        Ok(result)
    }

    pub fn get_app_detail(&self, process_name: &str, days: i64) -> Result<AppDetail> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        let start_date = (Local::now() - chrono::Duration::days(days)).format("%Y-%m-%d").to_string();

        // Daily trend
        let mut stmt = self.conn.prepare(
            "SELECT date, SUM(duration_sec) as total_sec, COUNT(DISTINCT process_name) as app_count
             FROM app_usage
             WHERE process_name = ?1 AND date >= ?2 AND date <= ?3
             GROUP BY date ORDER BY date DESC",
        )?;
        let daily_trend: Vec<DailySummary> = stmt.query_map(params![process_name, start_date, today], |row| {
            Ok(DailySummary {
                date: row.get(0)?,
                total_sec: row.get(1)?,
                app_count: row.get(2)?,
            })
        })?.collect::<Result<Vec<_>>>()?;

        // Window titles
        let mut stmt = self.conn.prepare(
            "SELECT window_title, SUM(duration_sec) as total_sec
             FROM app_usage
             WHERE process_name = ?1 AND date >= ?2 AND date <= ?3
             GROUP BY window_title ORDER BY total_sec DESC LIMIT 10",
        )?;
        let window_titles: Vec<(String, i64)> = stmt.query_map(params![process_name, start_date, today], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?.collect::<Result<Vec<_>>>()?;

        // Total
        let total_sec: i64 = daily_trend.iter().map(|d| d.total_sec).sum();
        let active_days = daily_trend.len().max(1) as f64;
        let avg_daily_sec = total_sec as f64 / active_days;

        Ok(AppDetail {
            process_name: process_name.to_string(),
            daily_trend,
            window_titles,
            total_sec,
            avg_daily_sec,
        })
    }

    pub fn get_comparison(&self, period: &str) -> Result<ComparisonData> {
        let now = Local::now();
        let (cur_start, cur_end, prev_start, prev_end) = match period {
            "week" => {
                let cur_end = now.format("%Y-%m-%d").to_string();
                let cur_start = (now - chrono::Duration::days(6)).format("%Y-%m-%d").to_string();
                let prev_end = (now - chrono::Duration::days(7)).format("%Y-%m-%d").to_string();
                let prev_start = (now - chrono::Duration::days(13)).format("%Y-%m-%d").to_string();
                (cur_start, cur_end, prev_start, prev_end)
            }
            _ => { // month
                let cur_start = now.format("%Y-%m-01").to_string();
                let cur_end = now.format("%Y-%m-%d").to_string();
                let prev_month = now.with_day(1).unwrap_or(now) - chrono::Duration::days(1);
                let prev_start = prev_month.format("%Y-%m-01").to_string();
                let prev_end = prev_month.format("%Y-%m-%d").to_string();
                (cur_start, cur_end, prev_start, prev_end)
            }
        };

        let (current_total, current_days): (i64, i64) = self.conn.query_row(
            "SELECT COALESCE(SUM(duration_sec), 0), COUNT(DISTINCT date) FROM app_usage WHERE date >= ?1 AND date <= ?2",
            params![cur_start, cur_end],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;

        let (previous_total, previous_days): (i64, i64) = self.conn.query_row(
            "SELECT COALESCE(SUM(duration_sec), 0), COUNT(DISTINCT date) FROM app_usage WHERE date >= ?1 AND date <= ?2",
            params![prev_start, prev_end],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;

        let current_avg = if current_days > 0 { current_total as f64 / current_days as f64 } else { 0.0 };
        let previous_avg = if previous_days > 0 { previous_total as f64 / previous_days as f64 } else { 0.0 };
        let change_pct = if previous_avg > 0.0 { ((current_avg - previous_avg) / previous_avg) * 100.0 } else { 0.0 };

        Ok(ComparisonData {
            current_total,
            previous_total,
            current_avg,
            previous_avg,
            current_days,
            previous_days,
            change_pct,
        })
    }

    pub fn cleanup_old_records(&mut self, keep_days: i64) -> Result<usize> {
        let cutoff = (Local::now() - chrono::Duration::days(keep_days)).format("%Y-%m-%d").to_string();
        let deleted = self.conn.execute(
            "DELETE FROM app_usage WHERE date < ?1",
            params![cutoff],
        )?;
        if deleted > 0 {
            self.today_cache.invalidate();
            self.weekly_cache.invalidate();
            self.monthly_cache.invalidate();
        }
        Ok(deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_escape_simple() {
        assert_eq!(csv_escape("hello"), "hello");
    }

    #[test]
    fn test_csv_escape_comma() {
        assert_eq!(csv_escape("hello,world"), "\"hello,world\"");
    }

    #[test]
    fn test_csv_escape_quote() {
        assert_eq!(csv_escape("say \"hi\""), "\"say \"\"hi\"\"\"");
    }

    #[test]
    fn test_csv_escape_newline() {
        assert_eq!(csv_escape("line1\nline2"), "\"line1\nline2\"");
    }

    #[test]
    fn test_query_cache_miss() {
        let cache = QueryCache::<i32>::new(5);
        assert!(cache.get().is_none());
    }

    #[test]
    fn test_query_cache_hit() {
        let mut cache = QueryCache::<i32>::new(5);
        cache.set(42);
        assert_eq!(cache.get(), Some(&42));
    }

    #[test]
    fn test_query_cache_invalidate() {
        let mut cache = QueryCache::<i32>::new(5);
        cache.set(42);
        cache.invalidate();
        assert!(cache.get().is_none());
    }

    #[test]
    fn test_database_init() {
        let db = rusqlite::Connection::open_in_memory().unwrap();
        db.execute_batch("PRAGMA journal_mode=WAL; PRAGMA auto_vacuum=INCREMENTAL;").unwrap();
        let database = Database {
            conn: db,
            today_cache: QueryCache::new(5),
            weekly_cache: QueryCache::new(30),
            monthly_cache: QueryCache::new(60),
            settings_cache: QueryCache::new(60),
        };
        database.init_tables().unwrap();

        // Settings should have defaults
        let settings = database.get_settings().unwrap();
        assert!(settings.autostart);
        assert!(settings.reminder_on_exit);
        assert_eq!(settings.daily_goal_minutes, 0);
        assert_eq!(settings.retention_days, 90);
        assert_eq!(settings.idle_threshold_minutes, 5);
    }

    #[test]
    fn test_insert_and_query() {
        let db = rusqlite::Connection::open_in_memory().unwrap();
        db.execute_batch("PRAGMA journal_mode=WAL;").unwrap();
        let mut database = Database {
            conn: db,
            today_cache: QueryCache::new(5),
            weekly_cache: QueryCache::new(30),
            monthly_cache: QueryCache::new(60),
            settings_cache: QueryCache::new(60),
        };
        database.init_tables().unwrap();

        let now = Local::now().naive_local();
        let id = database.insert_usage("test.exe", "Test Window", now, now, 60).unwrap();
        assert!(id > 0);

        let stats = database.get_today_stats(None).unwrap();
        assert_eq!(stats.total_sec, 60);
        assert_eq!(stats.app_count, 1);
        assert_eq!(stats.app_ranking.len(), 1);
        assert_eq!(stats.app_ranking[0].process_name, "test.exe");
    }
}
