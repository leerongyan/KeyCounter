"""Native application window based on pywebview."""

import threading
import time
from datetime import datetime

from . import settings
from .config import DATA_DIR

try:
    import webview
    WEBVIEW_AVAILABLE = True
except ImportError:
    webview = None
    WEBVIEW_AVAILABLE = False

_window = None
_force_close = False
_ask_pending = False


def _log(message):
    try:
        DATA_DIR.mkdir(parents=True, exist_ok=True)
        path = DATA_DIR / "panel.log"
        with open(path, "a", encoding="utf-8") as fh:
            fh.write(f"{datetime.now().isoformat(timespec='seconds')} {message}\n")
    except Exception:
        pass


def _after_close_decision(window, action):
    global _force_close
    global _ask_pending
    time.sleep(0.08)
    if _force_close:
        _ask_pending = False
        return
    current = settings.get("close_action", "ask")
    if current == "exit":
        _force_close = True
        _ask_pending = False
        window.destroy()
        return
    if current == "minimize":
        _ask_pending = False
        window.hide()
        return
    try:
        minimize = window.create_confirmation_dialog(
            "KeyCounter",
            "关闭窗口后将退出程序。\n\n是否改为最小化到托盘？\n选择“确定”最小化到托盘，“取消”直接退出。",
        )
    except Exception as exc:
        _log(f"ask dialog failed: {exc}")
        window.hide()
        _ask_pending = False
        return
    _ask_pending = False
    if minimize:
        window.hide()
    else:
        _force_close = True
        window.destroy()


def _handle_closing(window):
    global _force_close
    global _ask_pending
    if _force_close:
        return True
    action = settings.get("close_action", "ask")
    _log(f"closing action={action} pending={_ask_pending}")
    if action == "exit":
        return True
    if _ask_pending:
        return False
    _ask_pending = True
    threading.Thread(
        target=_after_close_decision,
        args=(window, action),
        daemon=True,
    ).start()
    return False


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
