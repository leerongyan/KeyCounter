import argparse
import threading
import webbrowser

from keycounter.config import DEFAULT_PORT
from keycounter.db import Database
from keycounter.server import DashboardServer
from keycounter.tracker import Tracker
from keycounter.tray import TrayController


def main():
    parser = argparse.ArgumentParser(description="键盘鼠标使用统计工具")
    parser.add_argument("--port", type=int, default=DEFAULT_PORT, help="仪表盘端口")
    parser.add_argument("--no-browser", action="store_true", help="启动后不自动打开浏览器")
    args = parser.parse_args()

    database = Database()
    tracker = Tracker(database)
    tracker.start()

    server = DashboardServer(("127.0.0.1", args.port), tracker)
    url = f"http://127.0.0.1:{args.port}"

    tray = TrayController(tracker, url, server.shutdown)
    if tray.start():
        print("系统托盘图标已启动")

    print("=" * 52)
    print("键盘鼠标使用统计工具已启动")
    print(f"仪表盘地址: {url}")
    print("按 Ctrl+C 停止统计，数据会自动保存")
    print("=" * 52)

    if not args.no_browser:
        threading.Timer(1.0, lambda: webbrowser.open(url)).start()

    try:
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
