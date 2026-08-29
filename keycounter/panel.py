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
_url = None
_force_close = False
_ask_pending = False
_hidden_close = False
_reopen = threading.Event()
_quit = threading.Event()


def _log(message):
    try:
        DATA_DIR.mkdir(parents=True, exist_ok=True)
        path = DATA_DIR / "panel.log"
        with open(path, "a", encoding="utf-8") as fh:
            fh.write(f"{datetime.now().isoformat(timespec='seconds')} {message}\n")
    except Exception:
        pass


def _hide_to_tray(window):
    """销毁窗口以释放 WebView2 占用的内存。

    托盘图标和服务仍然常驻，再次打开面板时会重建窗口。
    """
    global _hidden_close
    global _window
    _hidden_close = True
    _window = None
    try:
        window.destroy()
    except Exception:
        try:
            window.hide()
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
        _quit.set()
        window.destroy()
        return
    if current == "minimize":
        _ask_pending = False
        _hide_to_tray(window)
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
        _hide_to_tray(window)
    else:
        _force_close = True
        _quit.set()
        window.destroy()


def _handle_closing(window):
    global _force_close
    global _ask_pending
    if _force_close or _hidden_close:
        return True
    action = settings.get("close_action", "ask")
    _log(f"closing action={action} pending={_ask_pending}")
    if action == "exit":
        _quit.set()
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
    global _window, _url, _hidden_close
    _url = url
    _hidden_close = False
    if not WEBVIEW_AVAILABLE or _window is not None:
        return _window
    _window = webview.create_window(
        "KeyCounter 键盘鼠标统计",
        url=url,
        width=width,
        height=height,
        min_size=(720, 520),
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
    else:
        # 窗口收起到托盘时已被销毁，通知主循环重建
        _reopen.set()


def run(url):
    global _window
    if not WEBVIEW_AVAILABLE:
        return False
    while not _quit.is_set():
        if create_window(url) is None:
            return False
        webview.start()
        _window = None
        if _force_close or _quit.is_set():
            break
        # 窗口销毁只是收纳到了托盘：等待再次打开或退出
        while not (_reopen.is_set() or _quit.is_set()):
            time.sleep(0.1)
        _reopen.clear()
    return True


def close():
    global _force_close
    global _window
    _force_close = True
    _quit.set()
    _reopen.set()
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
