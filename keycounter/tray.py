import webbrowser

from . import autostart

try:
    import pystray
    from PIL import Image, ImageDraw, ImageFont
    TRAY_AVAILABLE = True
except ImportError:
    pystray = None
    TRAY_AVAILABLE = False


class TrayController:
    def __init__(self, tracker, url, on_quit):
        self.tracker = tracker
        self.url = url
        self.on_quit = on_quit
        self.icon = None

    def start(self):
        if not TRAY_AVAILABLE:
            return False
        menu = pystray.Menu(
            pystray.MenuItem(
                "打开统计面板",
                lambda icon, item: webbrowser.open(self.url),
                default=True,
            ),
            pystray.MenuItem("暂停 / 继续统计", self._toggle_pause),
            pystray.MenuItem(
                "开机自启",
                self._toggle_autostart,
                checked=lambda item: autostart.is_enabled(),
            ),
            pystray.MenuItem(
                "导出 CSV",
                lambda icon, item: webbrowser.open(self.url + "/api/export?format=csv"),
            ),
            pystray.MenuItem("退出", self._quit),
        )
        self.icon = pystray.Icon(
            "KeyCounter",
            self._create_image(),
            "键盘鼠标统计",
            menu,
        )
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

    def _quit(self, icon, item):
        icon.stop()
        self.on_quit()

    def _create_image(self):
        image = Image.new("RGBA", (64, 64), (0, 0, 0, 0))
        draw = ImageDraw.Draw(image)
        draw.rounded_rectangle([2, 2, 62, 62], radius=14, fill=(15, 118, 110, 255))
        try:
            font = ImageFont.truetype("segoeui.ttf", 24)
        except OSError:
            font = ImageFont.load_default()
        draw.text((32, 31), "KC", font=font, fill=(255, 255, 255, 255), anchor="mm")
        return image
