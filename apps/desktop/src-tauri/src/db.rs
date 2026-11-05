//! SQLite persistence for settings, inspection items and test history.
//! Local-only; no telemetry. Uses rusqlite with the bundled SQLite engine.

use crate::error::{AppError, AppResult};
use crate::model::{AppSettings, InspectionItem};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Mutex;

pub struct Db {
    conn: Mutex<Connection>,
}

impl Db {
    pub fn open(path: &Path) -> AppResult<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS settings (id INTEGER PRIMARY KEY CHECK (id = 1), data TEXT NOT NULL);
             CREATE TABLE IF NOT EXISTS inspection (id TEXT PRIMARY KEY, category TEXT, name TEXT, status TEXT, notes TEXT, automated INTEGER, updated_at_ms INTEGER);
             CREATE TABLE IF NOT EXISTS benchmark_history (id INTEGER PRIMARY KEY AUTOINCREMENT, kind TEXT, data TEXT, created_at_ms INTEGER);
             CREATE TABLE IF NOT EXISTS diagnostic_history (id INTEGER PRIMARY KEY AUTOINCREMENT, data TEXT, created_at_ms INTEGER);",
        )?;
        Ok(Self { conn: Mutex::new(conn) })
    }

    pub fn load_settings(&self) -> AppResult<AppSettings> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT data FROM settings WHERE id = 1")?;
        let mut rows = stmt.query([])?;
        if let Some(row) = rows.next()? {
            let data: String = row.get(0)?;
            match serde_json::from_str::<AppSettings>(&data) {
                Ok(s) => Ok(s),
                Err(_) => Ok(AppSettings::default()),
            }
        } else {
            Ok(AppSettings::default())
        }
    }

    pub fn save_settings(&self, settings: &AppSettings) -> AppResult<()> {
        let conn = self.conn.lock().unwrap();
        let data = serde_json::to_string(settings)?;
        conn.execute(
            "INSERT INTO settings (id, data) VALUES (1, ?1) ON CONFLICT(id) DO UPDATE SET data = ?1",
            params![data],
        )?;
        Ok(())
    }

    pub fn load_inspection(&self) -> AppResult<Vec<InspectionItem>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, category, name, status, notes, automated, updated_at_ms FROM inspection")?;
        let rows = stmt.query_map([], |row| {
            Ok(InspectionItem {
                id: row.get(0)?,
                category: row.get(1)?,
                name: row.get(2)?,
                status: row.get(3)?,
                notes: row.get(4)?,
                automated: row.get::<_, i64>(5)? != 0,
                updated_at_ms: row.get::<_, i64>(6)? as u64,
            })
        })?;
        let mut items = Vec::new();
        for r in rows {
            items.push(r?);
        }
        Ok(items)
    }

    pub fn upsert_inspection(&self, item: &InspectionItem) -> AppResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO inspection (id, category, name, status, notes, automated, updated_at_ms)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(id) DO UPDATE SET status = ?4, notes = ?5, updated_at_ms = ?7",
            params![item.id, item.category, item.name, item.status, item.notes, item.automated as i64, item.updated_at_ms as i64],
        )?;
        Ok(())
    }

    pub fn save_benchmark(&self, kind: &str, data: &str) -> AppResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO benchmark_history (kind, data, created_at_ms) VALUES (?1, ?2, ?3)",
            params![kind, data, crate::model::now_ms() as i64],
        )?;
        Ok(())
    }

    pub fn save_diagnostic(&self, data: &str) -> AppResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO diagnostic_history (data, created_at_ms) VALUES (?1, ?2)",
            params![data, crate::model::now_ms() as i64],
        )?;
        Ok(())
    }

    /// Most recent benchmark runs, newest first, as "[kind] json" summaries.
    pub fn recent_benchmarks(&self, limit: u32) -> AppResult<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT kind, data FROM benchmark_history ORDER BY id DESC LIMIT ?1")?;
        let rows = stmt.query_map(params![limit], |row| {
            let kind: String = row.get(0)?;
            let data: String = row.get(1)?;
            Ok(format!("[{kind}] {data}"))
        })?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }
}
