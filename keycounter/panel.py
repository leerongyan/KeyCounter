"""Native application window based on pywebview."""

from . import settings

try:
    import webview
    WEBVIEW_AVAILABLE = True
except ImportError:
    webview = None
    WEBVIEW_AVAILABLE = False

_window = None
_force_close = False


def _handle_closing(window):
    global _force_close
    if _force_close:
        return True
    action = settings.get("close_action", "ask")
    if action == "exit":
        return True
    if action == "minimize":
        window.hide()
        return False
    try:
        minimize = window.create_confirmation_dialog(
            "KeyCounter",
            "关闭窗口后将退出程序。\n\n是否改为最小化到托盘？\n选择“确定”最小化到托盘，“取消”直接退出。",
        )
    except Exception:
        return True
    if minimize:
        window.hide()
        return False
    return True


def create_window(url, width=1280, height=860):
    global _window
    if not WEBVIEW_AVAILABLE or _window is not None:
        return _window
    _window = webview.create_window(
        "KeyCounter 键盘鼠标统计",
        url=url,
        width=width,
        height=height,
        min_size=(900, 620),
        resizable=True,
    )
    _window.events.closing += lambda: _handle_closing(_window)
    return _window


def show():
    global _window
    if _window is not None:
        try:
            _window.show()
            _window.restore()
        except Exception:
            pass


def run(url):
    global _window
    if not WEBVIEW_AVAILABLE:
        return False
    create_window(url)
    if _window is None:
        return False
    webview.start()
    _window = None
    return True


def close():
    global _force_close
    global _window
    _force_close = True
    if _window is not None:
        try:
            _window.destroy()
        except Exception:
            pass
    _window = None
    _force_close = False
    if WEBVIEW_AVAILABLE:
        try:
            webview.destroy_window()
        except Exception:
            pass
