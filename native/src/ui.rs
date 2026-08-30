//! 窗口/WebView 管理与主事件循环。
//!
//! 关闭行为与 Python 版一致（close_action: ask/minimize/exit）；
//! minimize/ask-确认 时销毁窗口与 WebView（WebView2 内存随之释放），
//! 托盘"打开统计面板"时重新创建——Rust 常驻内核不受影响。

use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::{Icon as TaoIcon, Window, WindowBuilder};
use windows::Win32::UI::WindowsAndMessaging::{
    MessageBoxW, IDYES, MB_ICONQUESTION, MB_TOPMOST, MB_YESNO,
};

use crate::autostart;
use crate::exporter;
use crate::hooks;
use crate::settings;
use crate::store::Reader;
use crate::tray::{poll_action, TrayAction};

pub struct AppShared {
    pub reader: Reader,
    pub paused: Arc<std::sync::atomic::AtomicBool>,
    pub port: u16,
    pub no_window: bool,
}

struct Panel {
    window: Window,
    webview: wry::WebView,
}

impl Drop for Panel {
    fn drop(&mut self) {
        eprintln!("[debug] Panel dropped: webview + window released");
    }
}

/// 销毁面板：drop 释放 WebView2 controller 与原生窗口，
/// WebView2 浏览器进程随之退出，内存完全释放。
fn close_panel(panel: &Mutex<Option<Panel>>) {
    let _ = panel.lock().unwrap().take();
}

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn message_box_yesno(text: &str) -> bool {
    unsafe {
        let t = to_wide(text);
        let c = to_wide("KeyCounter");
        MessageBoxW(
            None,
            windows::core::PCWSTR(t.as_ptr()),
            windows::core::PCWSTR(c.as_ptr()),
            MB_YESNO | MB_ICONQUESTION | MB_TOPMOST,
        ) == IDYES
    }
}

fn message_box_ok(text: &str) {
    unsafe {
        let t = to_wide(text);
        let c = to_wide("KeyCounter");
        MessageBoxW(
            None,
            windows::core::PCWSTR(t.as_ptr()),
            windows::core::PCWSTR(c.as_ptr()),
            MB_TOPMOST,
        );
    }
}

fn app_icon() -> Option<TaoIcon> {
    let (rgba, w, h) = crate::heatmap::tray_icon_rgba()?;
    TaoIcon::from_rgba(rgba, w, h).ok()
}

pub fn run(shared: AppShared) -> Result<(), String> {
    let event_loop: EventLoop<UserEvent> =
        tao::event_loop::EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let tray = crate::tray::build().map_err(|e| format!("托盘初始化失败: {e}"))?;
    let window_icon = app_icon();

    let url = format!("http://127.0.0.1:{}/", shared.port);
    let panel: Arc<Mutex<Option<Panel>>> = Arc::new(Mutex::new(None));
    let quitting = Arc::new(std::sync::atomic::AtomicBool::new(false));

    if !shared.no_window {
        let _ = proxy.send_event(UserEvent::OpenPanel);
    }

    let panel_for_events = panel.clone();
    let quitting_for_events = quitting.clone();
    let proxy_for_events = proxy.clone();

    event_loop.run(move |event, _target, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(action) => match action {
                UserEvent::OpenPanel => {
                    if quitting_for_events.load(Ordering::SeqCst) {
                        return;
                    }
                    let mut guard = panel_for_events.lock().unwrap();
                    if guard.is_some() {
                        if let Some(p) = guard.as_ref() {
                            p.window.set_visible(true);
                            p.window.set_focus();
                        }
                        return;
                    }
                    let mut builder = WindowBuilder::new()
                        .with_title("KeyCounter 键盘鼠标统计")
                        .with_inner_size(LogicalSize::new(1280.0, 860.0))
                        .with_min_inner_size(LogicalSize::new(720.0, 520.0))
                        .with_resizable(true);
                    if let Some(icon) = window_icon.clone() {
                        builder = builder.with_window_icon(Some(icon));
                    }
                    match builder
                        .build(_target)
                        .map_err(|e| e.to_string())
                        .and_then(|window| {
                            let webview = wry::WebViewBuilder::new()
                                .with_url(&url)
                                .build(&window)
                                .map_err(|e| e.to_string())?;
                            Ok(Panel { window, webview })
                        }) {
                        Ok(p) => *guard = Some(p),
                        Err(e) => eprintln!("面板创建失败: {e}"),
                    }
                }
                UserEvent::TogglePause => {
                    let now = !shared.paused.load(Ordering::SeqCst);
                    shared.paused.store(now, Ordering::SeqCst);
                    hooks::set_paused(now);
                }
                UserEvent::ToggleAutostart => {
                    autostart::toggle();
                    // 勾选状态由菜单事件后的 update_menu 体现（tray-icon 自动重绘）
                    let _ = autostart::is_enabled();
                }
                UserEvent::ExportCsv => {
                    let reader = shared.reader.clone();
                    std::thread::spawn(move || {
                        match std::panic::catch_unwind(move || exporter::export_csv(&reader)) {
                            Ok(data) => {
                                let filename = format!(
                                    "KeyCounter_stats_{}.csv",
                                    chrono::Local::now().format("%Y-%m-%d")
                                );
                                let path = dirs_home().join(&filename);
                                match std::fs::write(&path, data.as_bytes()) {
                                    Ok(()) => message_box_ok(&format!("已导出：{}", path.display())),
                                    Err(e) => message_box_ok(&format!("导出失败：{e}")),
                                }
                            }
                            Err(_) => message_box_ok("导出失败"),
                        }
                    });
                }
                UserEvent::ExportPng => {
                    let reader = shared.reader.clone();
                    std::thread::spawn(move || {
                        let day = chrono::Local::now().format("%Y-%m-%d").to_string();
                        match std::panic::catch_unwind(move || exporter::heatmap_png(&reader, &day)) {
                            Ok(Some(bytes)) => {
                                let filename = format!(
                                    "KeyCounter_heatmap_{}.png",
                                    chrono::Local::now().format("%Y-%m-%d")
                                );
                                let path = dirs_home().join(&filename);
                                match std::fs::write(&path, bytes) {
                                    Ok(()) => message_box_ok(&format!("已导出：{}", path.display())),
                                    Err(e) => message_box_ok(&format!("导出失败：{e}")),
                                }
                            }
                            _ => message_box_ok("热力图生成失败"),
                        }
                    });
                }
                UserEvent::Quit => {
                    quitting_for_events.store(true, Ordering::SeqCst);
                    *control_flow = ControlFlow::Exit;
                }
            },
            Event::WindowEvent {
                window_id,
                event: WindowEvent::CloseRequested,
                ..
            } => {
                let owned = panel_for_events
                    .lock()
                    .unwrap()
                    .as_ref()
                    .map(|p| p.window.id() == window_id)
                    .unwrap_or(false);
                if !owned {
                    return;
                }
                match settings::get_close_action().as_str() {
                    "exit" => {
                        quitting_for_events.store(true, Ordering::SeqCst);
                        *control_flow = ControlFlow::Exit;
                    }
                    "minimize" => {
                        close_panel(&panel_for_events);
                    }
                    _ => {
                        // ask
                        let minimize = message_box_yesno(
                            "关闭窗口后将退出程序。\n\n是否改为最小化到托盘？\n选择\u{201c}确定\u{201d}最小化到托盘，\u{201c}取消\u{201d}直接退出。",
                        );
                        if minimize {
                            close_panel(&panel_for_events);
                        } else {
                            quitting_for_events.store(true, Ordering::SeqCst);
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                }
            }
            Event::NewEvents(_) => {
                // 每轮先轮询托盘事件
            }
            Event::MainEventsCleared => {
                while let Some(action) = poll_action(&tray) {
                    match action {
                        TrayAction::OpenPanel => {
                            let _ = proxy_for_events.send_event(UserEvent::OpenPanel);
                        }
                        TrayAction::TogglePause => {
                            let _ = proxy_for_events.send_event(UserEvent::TogglePause);
                        }
                        TrayAction::ToggleAutostart => {
                            let _ = proxy_for_events.send_event(UserEvent::ToggleAutostart);
                        }
                        TrayAction::ExportCsv => {
                            let _ = proxy_for_events.send_event(UserEvent::ExportCsv);
                        }
                        TrayAction::ExportPng => {
                            let _ = proxy_for_events.send_event(UserEvent::ExportPng);
                        }
                        TrayAction::Quit => {
                            let _ = proxy_for_events.send_event(UserEvent::Quit);
                        }
                    }
                }
            }
            Event::LoopDestroyed => {}
            _ => {}
        }
    });
}

#[derive(Debug, Clone, Copy)]
pub enum UserEvent {
    OpenPanel,
    TogglePause,
    ToggleAutostart,
    ExportCsv,
    ExportPng,
    Quit,
}

fn dirs_home() -> std::path::PathBuf {
    std::env::var_os("USERPROFILE")
        .map(std::path::PathBuf::from)
        .map(|p| p.join("Downloads"))
        .unwrap_or_else(|| std::path::PathBuf::from("."))
}
