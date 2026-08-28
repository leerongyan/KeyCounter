import argparse
import threading
import webbrowser

from keycounter.config import DEFAULT_PORT
from keycounter.db import Database
from keycounter.panel import WEBVIEW_AVAILABLE
from keycounter.panel import close as close_panel
from keycounter.panel import run as run_panel
from keycounter.panel import show as show_panel
from keycounter.server import DashboardServer
from keycounter.tracker import Tracker
from keycounter.tray import TrayController


def main():
    parser = argparse.ArgumentParser(description="键盘鼠标使用统计工具")
    parser.add_argument("--port", type=int, default=DEFAULT_PORT, help="本地服务端口")
    parser.add_argument("--no-browser", action="store_true", help="不打开统计面板或浏览器")
    args = parser.parse_args()

    database = Database()
    tracker = Tracker(database)
    tracker.start()

    server = DashboardServer(("127.0.0.1", args.port), tracker)
    url = f"http://127.0.0.1:{args.port}"

    def quit_app():
        server.shutdown()
        close_panel()

    tray = TrayController(tracker, url, quit_app, show_panel)
    if tray.start():
        print("系统托盘图标已启动")

    print("=" * 52)
    print("键盘鼠标使用统计工具已启动")
    print(f"本地服务: {url}")
    print("关闭统计窗口或按 Ctrl+C 会停止统计")
    print("=" * 52)

    try:
        if args.no_browser:
            server.serve_forever()
        elif WEBVIEW_AVAILABLE:
            print("正在打开统计面板...")
            threading.Thread(target=server.serve_forever, daemon=True).start()
            try:
                run_panel(url)
            except Exception as exc:
                print(f"统计面板打开失败：{exc}，将使用浏览器打开")
                threading.Timer(1.0, lambda: webbrowser.open(url)).start()
                server.serve_forever()
        else:
            print("未检测到原生窗口组件，使用浏览器打开统计面板")
            threading.Timer(1.0, lambda: webbrowser.open(url)).start()
            server.serve_forever()
    except KeyboardInterrupt:
        print("\n正在停止统计...")
    finally:
        tray.stop()
        server.shutdown()
        server.server_close()
        tracker.stop()
        print("已停止，数据已保存。")


if __name__ == "__main__":
    main()
