//! KeyCounter 原生版入口。

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
            "--help" => {
                println!("用法: KeyCounterNative [--port 8765] [--data-dir 目录] [--no-window]");
                return;
            }
            other => eprintln!("未知参数 {other}（--help 查看用法）"),
        }
    }

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    let data_dir = data_dir.unwrap_or_else(|| exe_dir.join("data"));
    if let Err(e) = std::fs::create_dir_all(&data_dir) {
        eprintln!("创建数据目录失败: {e}");
        std::process::exit(1);
    }
    settings::init(&data_dir);

    let db_path = data_dir.join("keycounter.db");
    let write_conn = match store::open_connection(&db_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("打开数据库失败: {e}");
            std::process::exit(1);
        }
    };
    let read_conn = match store::open_connection(&db_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("打开数据库失败: {e}");
            std::process::exit(1);
        }
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
    if let Err(e) = server::spawn(ctx, port) {
        eprintln!("HTTP 服务启动失败（端口 {port} 可能被占用）: {e}");
        std::process::exit(1);
    }
    println!("KeyCounter native 已启动: http://127.0.0.1:{port}/");

    let shared = ui::AppShared {
        reader,
        paused,
        port,
        no_window,
    };
    if let Err(e) = ui::run(shared) {
        eprintln!("{e}");
        std::process::exit(1);
    }

    // 事件循环退出：停钩子、清空写库队列、关库
    hook_stop.stop();
    writer.stop();
}
