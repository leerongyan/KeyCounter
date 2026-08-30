//! KeyCounter 原生版入口。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod autostart;
mod exporter;
mod heatmap;
mod hooks;
mod keys;
mod model;
mod server;
mod settings;
mod store;
mod tray;
mod ui;

use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

fn main() {
    let mut port = 8765u16;
    let mut data_dir: Option<PathBuf> = None;
    let mut no_window = false;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--port" => {
                port = args.next().and_then(|v| v.parse().ok()).unwrap_or(8765);
            }
            "--data-dir" => {
                data_dir = args.next().map(PathBuf::from);
            }
            "--no-window" => no_window = true,
            _ => {}
        }
    }

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    let data_dir = data_dir.unwrap_or_else(|| exe_dir.join("data"));
    if std::fs::create_dir_all(&data_dir).is_err() {
        std::process::exit(1);
    }
    settings::init(&data_dir);

    let db_path = data_dir.join("keycounter.db");
    let write_conn = match store::open_connection(&db_path) {
        Ok(c) => c,
        Err(_) => std::process::exit(1),
    };
    let read_conn = match store::open_connection(&db_path) {
        Ok(c) => c,
        Err(_) => std::process::exit(1),
    };

    let (tx, rx) = crossbeam_channel::unbounded();
    let hook_stop = hooks::start(tx);
    let mut writer = store::Writer::spawn(write_conn, rx);
    let reader = store::Reader::new(read_conn);
    let paused = Arc::new(AtomicBool::new(false));

    let ctx = Arc::new(server::ServerContext {
        reader: reader.clone(),
        paused: paused.clone(),
        started_at: model::now_text(),
    });
    if server::spawn(ctx, port).is_err() {
        // 端口被占用（可能已有实例在运行）时静默退出，避免弹控制台报错
        std::process::exit(1);
    }

    let shared = ui::AppShared {
        reader,
        paused,
        port,
        no_window,
    };
    if ui::run(shared).is_err() {
        std::process::exit(1);
    }

    // 事件循环退出：停钩子、清空写库队列、关库
    hook_stop.stop();
    writer.stop();
}
