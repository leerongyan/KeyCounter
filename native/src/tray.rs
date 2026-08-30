//! 托盘图标与菜单（tray-icon）。菜单事件由 main 的事件循环轮询。

use tray_icon::menu::{CheckMenuItem, Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem};
use tray_icon::{TrayIcon, TrayIconBuilder, TrayIconEvent};

use crate::autostart;
use crate::heatmap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayAction {
    OpenPanel,
    TogglePause,
    ToggleAutostart,
    ExportCsv,
    ExportPng,
    Quit,
}

pub struct Tray {
    #[allow(dead_code)] // 必须持有 TrayIcon，drop 会移除托盘图标
    pub icon: TrayIcon,
    pub ids: MenuIds,
}

pub struct MenuIds {
    pub open: MenuId,
    pub pause: MenuId,
    pub autostart: MenuId,
    pub export_csv: MenuId,
    pub export_png: MenuId,
    pub quit: MenuId,
}

pub fn build() -> Result<Tray, String> {
    let (rgba, w, h) = heatmap::tray_icon_rgba().ok_or("tray icon render failed")?;
    let icon = tray_icon::Icon::from_rgba(rgba, w, h).map_err(|e| e.to_string())?;

    let open = MenuItem::new("打开统计面板", true, None);
    let pause = MenuItem::new("暂停 / 继续统计", true, None);
    let autostart_item = CheckMenuItem::new("开机自启", true, autostart::is_enabled(), None);
    let export_csv = MenuItem::new("导出 CSV 文件", true, None);
    let export_png = MenuItem::new("导出热力图 PNG", true, None);
    let quit = MenuItem::new("退出", true, None);

    let menu = Menu::new();
    menu.append(&open).map_err(|e| e.to_string())?;
    menu.append(&pause).map_err(|e| e.to_string())?;
    menu.append(&autostart_item).map_err(|e| e.to_string())?;
    menu.append(&PredefinedMenuItem::separator()).map_err(|e| e.to_string())?;
    menu.append(&export_csv).map_err(|e| e.to_string())?;
    menu.append(&export_png).map_err(|e| e.to_string())?;
    menu.append(&PredefinedMenuItem::separator()).map_err(|e| e.to_string())?;
    menu.append(&quit).map_err(|e| e.to_string())?;

    let ids = MenuIds {
        open: open.id().clone(),
        pause: pause.id().clone(),
        autostart: autostart_item.id().clone(),
        export_csv: export_csv.id().clone(),
        export_png: export_png.id().clone(),
        quit: quit.id().clone(),
    };

    let icon = TrayIconBuilder::new()
        .with_tooltip("键盘鼠标统计")
        .with_icon(icon)
        .with_menu(Box::new(menu))
        .with_menu_on_left_click(false)
        .build()
        .map_err(|e| e.to_string())?;

    Ok(Tray { icon, ids })
}

/// 非阻塞轮询一次托盘/菜单事件。
pub fn poll_action(tray: &Tray) -> Option<TrayAction> {
    if let Ok(event) = MenuEvent::receiver().try_recv() {
        let id: &MenuId = event.id();
        return if id == &tray.ids.open {
            Some(TrayAction::OpenPanel)
        } else if id == &tray.ids.pause {
            Some(TrayAction::TogglePause)
        } else if id == &tray.ids.autostart {
            Some(TrayAction::ToggleAutostart)
        } else if id == &tray.ids.export_csv {
            Some(TrayAction::ExportCsv)
        } else if id == &tray.ids.export_png {
            Some(TrayAction::ExportPng)
        } else if id == &tray.ids.quit {
            Some(TrayAction::Quit)
        } else {
            None
        };
    }
    if let Ok(event) = TrayIconEvent::receiver().try_recv() {
        if matches!(
            event,
            TrayIconEvent::Click {
                button: tray_icon::MouseButton::Left,
                ..
            }
        ) {
            return Some(TrayAction::OpenPanel);
        }
    }
    None
}
