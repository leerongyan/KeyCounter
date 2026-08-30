//! SQLite 存储层：schema、批量写入线程、查询接口。
//! 表结构与 SQL 聚合与 Python 版 db.py / tracker.py 完全一致。

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crossbeam_channel::Receiver;
use rusqlite::Connection;

use crate::model::Event;

pub fn open_connection(path: &std::path::Path) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(
        "PRAGMA journal_mode=WAL;
         CREATE TABLE IF NOT EXISTS key_events (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             key_name TEXT NOT NULL,
             pressed_at TEXT NOT NULL
         );
         CREATE TABLE IF NOT EXISTS mouse_events (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             event_type TEXT NOT NULL,
             button TEXT NOT NULL DEFAULT '',
             x INTEGER NOT NULL DEFAULT 0,
             y INTEGER NOT NULL DEFAULT 0,
             pressed_at TEXT NOT NULL
         );
         CREATE TABLE IF NOT EXISTS mouse_moves (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             x INTEGER NOT NULL,
             y INTEGER NOT NULL,
             distance_px REAL NOT NULL DEFAULT 0,
             sampled_at TEXT NOT NULL
         );
         CREATE INDEX IF NOT EXISTS idx_key_events_time ON key_events(pressed_at);
         CREATE INDEX IF NOT EXISTS idx_mouse_events_time ON mouse_events(pressed_at);
         CREATE INDEX IF NOT EXISTS idx_mouse_moves_time ON mouse_moves(sampled_at);",
    )?;
    Ok(conn)
}

pub struct Writer {
    handle: Option<std::thread::JoinHandle<()>>,
    stop: Arc<AtomicBool>,
}

impl Writer {
    /// 启动批量写库线程：0.4s 或攒满 500 条刷一次，单次事务提交。
    pub fn spawn(mut conn: Connection, rx: Receiver<Event>) -> Writer {
        let stop = Arc::new(AtomicBool::new(false));
        let stop_flag = stop.clone();
        let handle = std::thread::Builder::new()
            .name("kc-db-writer".into())
            .spawn(move || {
                while !stop_flag.load(Ordering::SeqCst) {
                    let mut batch = Vec::new();
                    match rx.recv_timeout(Duration::from_millis(400)) {
                        Ok(ev) => batch.push(ev),
                        Err(crossbeam_channel::RecvTimeoutError::Timeout) => {}
                        Err(crossbeam_channel::RecvTimeoutError::Disconnected) => break,
                    }
                    while batch.len() < 500 {
                        match rx.try_recv() {
                            Ok(ev) => batch.push(ev),
                            Err(_) => break,
                        }
                    }
                    if !batch.is_empty() {
                        write_batch(&mut conn, &batch);
                    }
                }
                // 退出前清空队列
                let mut rest = Vec::new();
                while let Ok(ev) = rx.try_recv() {
                    rest.push(ev);
                }
                if !rest.is_empty() {
                    write_batch(&mut conn, &rest);
                }
            })
            .expect("spawn db writer");
        Writer {
            handle: Some(handle),
            stop,
        }
    }

    pub fn stop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn write_batch(conn: &mut Connection, batch: &[Event]) {
    let mut keys: Vec<(String, &str)> = Vec::new();
    let mut mouse: Vec<(&str, &str, i32, i32, &str)> = Vec::new();
    let mut moves: Vec<(i32, i32, f64, &str)> = Vec::new();
    for ev in batch {
        match ev {
            Event::Key { name, at } => keys.push((name.clone(), at)),
            Event::Click { button, x, y, at } => {
                mouse.push(("click", button, *x, *y, at))
            }
            Event::Scroll { button, x, y, at } => {
                mouse.push(("scroll", button, *x, *y, at))
            }
            Event::Move { x, y, distance, at } => moves.push((*x, *y, *distance, at)),
        }
    }
    let tx = match conn.transaction() {
        Ok(tx) => tx,
        Err(_) => return,
    };
    let _ = tx.execute_batch("BEGIN");
    for (name, at) in &keys {
        let _ = tx.execute(
            "INSERT INTO key_events (key_name, pressed_at) VALUES (?1, ?2)",
            rusqlite::params![name, at],
        );
    }
    for (kind, button, x, y, at) in &mouse {
        let _ = tx.execute(
            "INSERT INTO mouse_events (event_type, button, x, y, pressed_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![kind, button, x, y, at],
        );
    }
    for (x, y, d, at) in &moves {
        let _ = tx.execute(
            "INSERT INTO mouse_moves (x, y, distance_px, sampled_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![x, y, d, at],
        );
    }
    let _ = tx.commit();
}

/// 只读连接封装（WAL 允许与写线程并发读）。
pub struct Reader {
    conn: Arc<Mutex<Connection>>,
}

impl Clone for Reader {
    fn clone(&self) -> Self {
        Reader {
            conn: self.conn.clone(),
        }
    }
}

impl Reader {
    pub fn new(conn: Connection) -> Reader {
        Reader {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    pub fn key_count_range(&self, s: &str, e: &str) -> i64 {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT COUNT(*) FROM key_events WHERE pressed_at BETWEEN ?1 AND ?2",
            rusqlite::params![s, e],
            |r| r.get(0),
        )
        .unwrap_or(0)
    }

    pub fn mouse_count_range(&self, s: &str, e: &str, kind: &str) -> i64 {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT COUNT(*) FROM mouse_events WHERE pressed_at BETWEEN ?1 AND ?2 AND event_type = ?3",
            rusqlite::params![s, e, kind],
            |r| r.get(0),
        )
        .unwrap_or(0)
    }

    pub fn distance_range(&self, s: &str, e: &str) -> f64 {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT COALESCE(SUM(distance_px), 0) FROM mouse_moves WHERE sampled_at BETWEEN ?1 AND ?2",
            rusqlite::params![s, e],
            |r| r.get(0),
        )
        .unwrap_or(0.0)
    }

    pub fn key_counts(&self, s: &str, e: &str, limit: Option<i64>) -> Vec<(String, i64)> {
        let conn = self.conn.lock().unwrap();
        let mut sql = "SELECT key_name, COUNT(*) AS count FROM key_events \
                       WHERE pressed_at BETWEEN ?1 AND ?2 GROUP BY key_name ORDER BY count DESC"
            .to_string();
        if let Some(n) = limit {
            sql.push_str(&format!(" LIMIT {n}"));
        }
        let mut stmt = match conn.prepare(&sql) {
            Ok(stmt) => stmt,
            Err(_) => return Vec::new(),
        };
        let rows = stmt.query_map(rusqlite::params![s, e], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?))
        });
        match rows {
            Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
            Err(_) => Vec::new(),
        }
    }

    pub fn mouse_counts(&self, s: &str, e: &str) -> Vec<(String, String, i64)> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = match conn.prepare(
            "SELECT event_type, button, COUNT(*) AS count FROM mouse_events \
             WHERE pressed_at BETWEEN ?1 AND ?2 GROUP BY event_type, button ORDER BY count DESC",
        ) {
            Ok(stmt) => stmt,
            Err(_) => return Vec::new(),
        };
        let rows = stmt.query_map(rusqlite::params![s, e], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, i64>(2)?))
        });
        match rows {
            Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
            Err(_) => Vec::new(),
        }
    }

    fn hourly(&self, s: &str, e: &str, sql: &str) -> Vec<(String, f64)> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = match conn.prepare(sql) {
            Ok(stmt) => stmt,
            Err(_) => return Vec::new(),
        };
        let rows = stmt.query_map(rusqlite::params![s, e], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, f64>(1)?))
        });
        match rows {
            Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
            Err(_) => Vec::new(),
        }
    }

    pub fn hourly_keys(&self, s: &str, e: &str) -> Vec<(String, i64)> {
        self.hourly(
            s,
            e,
            "SELECT substr(pressed_at, 12, 2) AS hour, COUNT(*) FROM key_events \
             WHERE pressed_at BETWEEN ?1 AND ?2 GROUP BY substr(pressed_at, 12, 2)",
        )
        .into_iter()
        .map(|(h, c)| (h, c as i64))
        .collect()
    }

    pub fn hourly_clicks(&self, s: &str, e: &str) -> Vec<(String, i64)> {
        self.hourly(
            s,
            e,
            "SELECT substr(pressed_at, 12, 2) AS hour, COUNT(*) FROM mouse_events \
             WHERE pressed_at BETWEEN ?1 AND ?2 AND event_type = 'click' \
             GROUP BY substr(pressed_at, 12, 2)",
        )
        .into_iter()
        .map(|(h, c)| (h, c as i64))
        .collect()
    }

    pub fn hourly_distance(&self, s: &str, e: &str) -> Vec<(String, f64)> {
        self.hourly(
            s,
            e,
            "SELECT substr(sampled_at, 12, 2) AS hour, COALESCE(SUM(distance_px), 0) \
             FROM mouse_moves WHERE sampled_at BETWEEN ?1 AND ?2 \
             GROUP BY substr(sampled_at, 12, 2)",
        )
    }

    pub fn daily_keys(&self, s: &str, e: &str) -> Vec<(String, i64)> {
        self.hourly(
            s,
            e,
            "SELECT substr(pressed_at, 1, 10) AS day, COUNT(*) FROM key_events \
             WHERE pressed_at BETWEEN ?1 AND ?2 GROUP BY substr(pressed_at, 1, 10)",
        )
        .into_iter()
        .map(|(d, c)| (d, c as i64))
        .collect()
    }

    pub fn daily_clicks(&self, s: &str, e: &str) -> Vec<(String, i64)> {
        self.hourly(
            s,
            e,
            "SELECT substr(pressed_at, 1, 10) AS day, COUNT(*) FROM mouse_events \
             WHERE pressed_at BETWEEN ?1 AND ?2 AND event_type = 'click' \
             GROUP BY substr(pressed_at, 1, 10)",
        )
        .into_iter()
        .map(|(d, c)| (d, c as i64))
        .collect()
    }

    pub fn daily_distance(&self, s: &str, e: &str) -> Vec<(String, f64)> {
        self.hourly(
            s,
            e,
            "SELECT substr(sampled_at, 1, 10) AS day, COALESCE(SUM(distance_px), 0) \
             FROM mouse_moves WHERE sampled_at BETWEEN ?1 AND ?2 \
             GROUP BY substr(sampled_at, 1, 10)",
        )
    }

    pub fn key_counts_all(&self) -> Vec<(String, i64)> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = match conn.prepare(
            "SELECT key_name, COUNT(*) AS count FROM key_events GROUP BY key_name ORDER BY count DESC",
        ) {
            Ok(stmt) => stmt,
            Err(_) => return Vec::new(),
        };
        let rows = stmt.query_map([], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?))
        });
        match rows {
            Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
            Err(_) => Vec::new(),
        }
    }

    /// 每日汇总（日期, 按键, 点击, 滚轮, 距离px），与 Python daily_rows 一致。
    pub fn daily_rows(&self) -> Vec<(String, i64, i64, i64, f64)> {
        let conn = self.conn.lock().unwrap();
        let mut stats: std::collections::BTreeMap<String, [f64; 4]> = Default::default();
        let queries: [(&str, usize); 3] = [
            (
                "SELECT substr(pressed_at, 1, 10) AS d, COUNT(*) FROM key_events GROUP BY d",
                0,
            ),
            (
                "SELECT substr(pressed_at, 1, 10) AS d, event_type, COUNT(*) FROM mouse_events GROUP BY d, event_type",
                1,
            ),
            (
                "SELECT substr(sampled_at, 1, 10) AS d, COALESCE(SUM(distance_px), 0) FROM mouse_moves GROUP BY d",
                3,
            ),
        ];
        let (q0, _) = queries[0];
        if let Ok(mut stmt) = conn.prepare(q0) {
            if let Ok(rows) = stmt.query_map([], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?))
            }) {
                for row in rows.flatten() {
                    stats.entry(row.0).or_default()[0] += row.1 as f64;
                }
            }
        }
        let (q1, _) = queries[1];
        if let Ok(mut stmt) = conn.prepare(q1) {
            if let Ok(rows) = stmt.query_map([], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, i64>(2)?,
                ))
            }) {
                for row in rows.flatten() {
                    let slot = match row.1.as_str() {
                        "click" => 1,
                        "scroll" => 2,
                        _ => continue,
                    };
                    stats.entry(row.0).or_default()[slot] += row.2 as f64;
                }
            }
        }
        let (q2, _) = queries[2];
        if let Ok(mut stmt) = conn.prepare(q2) {
            if let Ok(rows) = stmt.query_map([], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, f64>(1)?))
            }) {
                for row in rows.flatten() {
                    stats.entry(row.0).or_default()[3] += row.1;
                }
            }
        }
        stats
            .into_iter()
            .map(|(day, v)| {
                (
                    day,
                    v[0] as i64,
                    v[1] as i64,
                    v[2] as i64,
                    v[3],
                )
            })
            .collect()
    }
}
