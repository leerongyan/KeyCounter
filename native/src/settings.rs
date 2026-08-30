//! 设置存储：data/settings.json，与 Python 版同路径同格式。

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

static SETTINGS: Mutex<Option<PathBuf>> = Mutex::new(None);

pub fn init(data_dir: &Path) {
    let _ = SETTINGS.lock().unwrap().replace(data_dir.join("settings.json"));
}

fn path() -> PathBuf {
    SETTINGS
        .lock()
        .unwrap()
        .clone()
        .unwrap_or_else(|| PathBuf::from("data/settings.json"))
}

fn read_all() -> BTreeMap<String, serde_json::Value> {
    let p = path();
    if let Ok(text) = std::fs::read_to_string(&p) {
        if let Ok(v) = serde_json::from_str::<BTreeMap<String, serde_json::Value>>(&text) {
            return v;
        }
    }
    BTreeMap::new()
}

pub fn get_all() -> BTreeMap<String, serde_json::Value> {
    read_all()
}

pub fn get_close_action() -> String {
    match read_all().get("close_action") {
        Some(serde_json::Value::String(s)) if matches!(s.as_str(), "ask" | "minimize" | "exit") => {
            s.clone()
        }
        _ => "ask".to_string(),
    }
}

pub fn set(key: &str, value: serde_json::Value) {
    let p = path();
    if let Some(parent) = p.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let mut all = read_all();
    all.insert(key.to_string(), value);
    if let Ok(text) = serde_json::to_string_pretty(&all) {
        let _ = std::fs::write(&p, text);
    }
}
