import ctypes
import datetime
import sys
from pathlib import Path

from . import autostart

try:
    import pystray
    TRAY_AVAILABLE = True
except ImportError:
    pystray = None
    TRAY_AVAILABLE = False


def _icon_path():
    if getattr(sys, "frozen", False):
        base = Path(sys._MEIPASS) / "assets"
    else:
        base = Path(__file__).resolve().parent / "assets"
    return base / "icon.ico"


def _load_icon_handle(path):
    """用系统 API 从 .ico 文件加载图标句柄（不经过 PIL）。"""
    if sys.platform != "win32":
        return None
    try:
        IMAGE_ICON = 1
        LR_LOADFROMFILE = 0x10
        handle = ctypes.windll.user32.LoadImageW(
            None, str(path), IMAGE_ICON, 0, 0, LR_LOADFROMFILE)
        return handle or None
    except Exception:
        return None


if TRAY_AVAILABLE:

    class _PreloadedIcon(pystray.Icon):
        """预加载 .ico 句柄的托盘图标。

        pystray 默认把图标交给 PIL 转换成 .ico 再加载；这里改为直接用
        系统 LoadImage 从打包好的图标文件加载，启动时完全不需要 PIL。
        """

        def __init__(self, icon_path, *args, **kwargs):
            super().__init__(*args, **kwargs)
            self._icon_path = icon_path

        def _assert_icon_handle(self):
            if not self._icon_handle:
                handle = _load_icon_handle(self._icon_path)
                if handle:
                    self._icon_handle = handle
                    return
            # 图标文件缺失时退回 pystray 原路径（PIL 兜底）
            super()._assert_icon_handle()


class TrayController:
    def __init__(self, tracker, url, on_quit, on_open_panel):
        self.tracker = tracker
        self.url = url
        self.on_quit = on_quit
        self.on_open_panel = on_open_panel
        self.icon = None

    def start(self):
        if not TRAY_AVAILABLE:
            return False
        menu = pystray.Menu(
            pystray.MenuItem(
                "打开统计面板",
                lambda icon, item: self.on_open_panel(),
                default=True,
            ),
            pystray.MenuItem("暂停 / 继续统计", self._toggle_pause),
            pystray.MenuItem(
                "开机自启",
                self._toggle_autostart,
                checked=lambda item: autostart.is_enabled(),
            ),
            pystray.MenuItem("导出 CSV 文件", self._export_csv),
            pystray.MenuItem("导出热力图 PNG", self._export_heatmap),
            pystray.MenuItem("退出", self._quit),
        )
        self.icon = _PreloadedIcon(
            _icon_path(), "KeyCounter", True, "键盘鼠标统计", menu)
        self.icon.run_detached()
        return True

    def stop(self):
        if self.icon is not None:
            self.icon.stop()
            self.icon = None

    def _toggle_pause(self, icon, item):
        self.tracker.set_paused(not self.tracker.paused)
        if self.icon is not None:
            self.icon.update_menu()

    def _toggle_autostart(self, icon, item):
        autostart.toggle()
        if self.icon is not None:
            self.icon.update_menu()

    def _export_csv(self, icon, item):
        try:
            data = self.tracker.export_csv()
            filename = f"KeyCounter_stats_{datetime.date.today().isoformat()}.csv"
            path = Path.home() / "Downloads" / filename
            path.write_bytes(data)
            icon.notify(f"已导出：{path}", "KeyCounter")
        except Exception:
            pass

    def _export_heatmap(self, icon, item):
        try:
            data = self.tracker.heatmap_png()
            if not data:
                return
            filename = f"KeyCounter_heatmap_{datetime.date.today().isoformat()}.png"
            path = Path.home() / "Downloads" / filename
            path.write_bytes(data)
            icon.notify(f"已导出：{path}", "KeyCounter")
        except Exception:
            pass

    def _quit(self, icon, item):
        icon.stop()
        self.on_quit()

    def _create_image(self):
        # 仅在预加载失败时的兜底路径，正常启动不会加载 PIL
        from PIL import Image, ImageDraw, ImageFont

        image = Image.new("RGBA", (64, 64), (0, 0, 0, 0))
        draw = ImageDraw.Draw(image)
        draw.rounded_rectangle([2, 2, 62, 62], radius=14, fill=(15, 118, 110, 255))
        try:
            font = ImageFont.truetype("segoeui.ttf", 24)
        except OSError:
            font = ImageFont.load_default()
        draw.text((32, 31), "KC", font=font, fill=(255, 255, 255, 255), anchor="mm")
        return image
