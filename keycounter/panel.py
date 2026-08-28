"""Native application window based on pywebview."""

try:
    import webview
    WEBVIEW_AVAILABLE = True
except ImportError:
    webview = None
    WEBVIEW_AVAILABLE = False

_window = None


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
    global _window
    if _window is not None:
        try:
            _window.destroy()
        except Exception:
            pass
    _window = None
    if WEBVIEW_AVAILABLE:
        try:
            webview.destroy_window()
        except Exception:
            pass
