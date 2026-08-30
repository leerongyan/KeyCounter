//! 低级键盘/鼠标钩子：回调只把事件放进 channel，绝不触碰磁盘。

use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

use crossbeam_channel::Sender;
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, PostThreadMessageW, SetWindowsHookExW, UnhookWindowsHookEx,
    WH_KEYBOARD_LL, WH_MOUSE_LL, WM_QUIT,
};
use windows::Win32::UI::WindowsAndMessaging::{
    KBDLLHOOKSTRUCT, MSLLHOOKSTRUCT, MSG, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_MOUSEMOVE,
    WM_MOUSEWHEEL, WM_MBUTTONDOWN, WM_RBUTTONDOWN, WM_SYSKEYDOWN, WM_SYSKEYUP, WM_XBUTTONDOWN,
};

use crate::keys::normalize_vk;
use crate::model::{now_text, Event};

static EVENT_TX: OnceLock<Sender<Event>> = OnceLock::new();
static HELD_KEYS: OnceLock<Mutex<HashSet<u32>>> = OnceLock::new();
static LAST_MOVE: OnceLock<Mutex<Option<(i32, i32, Instant)>>> = OnceLock::new();
static PAUSED: AtomicBool = AtomicBool::new(false);

pub fn set_paused(paused: bool) {
    PAUSED.store(paused, Ordering::SeqCst);
    if !paused {
        // 与 Python 一致：恢复时清空移动基点和按住集合
        *last_move() = None;
        held_keys().clear();
    }
}

fn held_keys() -> std::sync::MutexGuard<'static, HashSet<u32>> {
    HELD_KEYS
        .get_or_init(|| Mutex::new(HashSet::new()))
        .lock()
        .unwrap()
}

fn last_move() -> std::sync::MutexGuard<'static, Option<(i32, i32, Instant)>> {
    LAST_MOVE
        .get_or_init(|| Mutex::new(None))
        .lock()
        .unwrap()
}

fn send(event: Event) {
    if let Some(tx) = EVENT_TX.get() {
        let _ = tx.send(event);
    }
}

unsafe extern "system" fn keyboard_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        let info = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
        match wparam.0 as u32 {
            m if m == WM_KEYDOWN || m == WM_SYSKEYDOWN => {
                if !PAUSED.load(Ordering::SeqCst) && held_keys().insert(info.vkCode) {
                    send(Event::Key {
                        name: normalize_vk(info.vkCode),
                        at: now_text(),
                    });
                }
            }
            m if m == WM_KEYUP || m == WM_SYSKEYUP => {
                held_keys().remove(&info.vkCode);
            }
            _ => {}
        }
    }
    CallNextHookEx(None, code, wparam, lparam)
}

unsafe extern "system" fn mouse_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        let info = &*(lparam.0 as *const MSLLHOOKSTRUCT);
        match wparam.0 as u32 {
            m if m == WM_LBUTTONDOWN => send(Event::Click {
                button: "left".to_string(),
                x: info.pt.x,
                y: info.pt.y,
                at: now_text(),
            }),
            m if m == WM_RBUTTONDOWN => send(Event::Click {
                button: "right".to_string(),
                x: info.pt.x,
                y: info.pt.y,
                at: now_text(),
            }),
            m if m == WM_MBUTTONDOWN => send(Event::Click {
                button: "middle".to_string(),
                x: info.pt.x,
                y: info.pt.y,
                at: now_text(),
            }),
            m if m == WM_XBUTTONDOWN => send(Event::Click {
                // 与 Python 一致：x1/x2/side 全部合并记为 side
                button: "side".to_string(),
                x: info.pt.x,
                y: info.pt.y,
                at: now_text(),
            }),
            m if m == WM_MOUSEWHEEL => {
                let delta = ((info.mouseData >> 16) & 0xFFFF) as u16 as i16;
                // 与 Python 版一致的方向映射：delta < 0 → "up"，其余 → "down"
                let button = if delta < 0 { "up" } else { "down" };
                send(Event::Scroll {
                    button: button.to_string(),
                    x: info.pt.x,
                    y: info.pt.y,
                    at: now_text(),
                });
            }
            m if m == WM_MOUSEMOVE => {
                let (x, y) = (info.pt.x, info.pt.y);
                let now = Instant::now();
                let mut last = last_move();
                if let Some((lx, ly, lt)) = *last {
                    let elapsed = now.saturating_duration_since(lt).as_secs_f64();
                    let distance = ((x - lx) * (x - lx) + (y - ly) * (y - ly)) as f64;
                    let distance = distance.sqrt();
                    if (0.02..=5.0).contains(&elapsed) && distance >= 2.0 {
                        send(Event::Move {
                            x,
                            y,
                            distance,
                            at: now_text(),
                        });
                    }
                }
                *last = Some((x, y, now));
            }
            _ => {}
        }
    }
    CallNextHookEx(None, code, wparam, lparam)
}

/// 启动钩子线程（内部消息泵），返回用于退出的句柄。
pub fn start(tx: Sender<Event>) -> HookStop {
    let _ = EVENT_TX.set(tx);
    let _ = HELD_KEYS.set(Mutex::new(HashSet::new()));
    let _ = LAST_MOVE.set(Mutex::new(None));

    let (id_tx, id_rx) = mpsc::channel();
    std::thread::Builder::new()
        .name("hooks".into())
        .spawn(move || unsafe {
            let hmodule = GetModuleHandleW(None).unwrap_or_default();
            let hinstance: windows::Win32::Foundation::HINSTANCE = hmodule.into();
            let kb = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), hinstance, 0);
            let ms = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_proc), hinstance, 0);
            if let (Ok(kb), Ok(ms)) = (kb, ms) {
                let _ = id_tx.send(GetCurrentThreadId());
                let mut msg = MSG::default();
                // GetMessageW 返回 -1 表示错误，0 表示 WM_QUIT
                while GetMessageW(&mut msg, None, 0, 0).0 > 0 {}
                let _ = UnhookWindowsHookEx(kb);
                let _ = UnhookWindowsHookEx(ms);
            }
        })
        .expect("spawn hook thread");

    // 等待钩子线程就绪并拿到线程 id（毫秒级）
    let thread_id = id_rx.recv_timeout(std::time::Duration::from_secs(2)).ok();
    HookStop { thread_id }
}

pub struct HookStop {
    thread_id: Option<u32>,
}

impl HookStop {
    pub fn stop(&self) {
        if let Some(id) = self.thread_id {
            unsafe {
                let _ = PostThreadMessageW(id, WM_QUIT, WPARAM(0), LPARAM(0));
            }
        }
    }
}
