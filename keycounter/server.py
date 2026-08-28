import json
import urllib.parse
from datetime import date
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

from . import autostart, settings
from .config import WEB_DIR

CONTENT_TYPES = {
    ".html": "text/html; charset=utf-8",
    ".css": "text/css; charset=utf-8",
    ".js": "text/javascript; charset=utf-8",
    ".svg": "image/svg+xml",
    ".png": "image/png",
    ".ico": "image/x-icon",
}


class DashboardServer(ThreadingHTTPServer):
    daemon_threads = True
    allow_reuse_address = True

    def __init__(self, server_address, tracker):
        self.tracker = tracker
        super().__init__(server_address, DashboardHandler)


class DashboardHandler(BaseHTTPRequestHandler):
    server_version = "KeyCounter/0.1"

    def do_GET(self):
        parsed = urllib.parse.urlparse(self.path)
        path = parsed.path

        if path == "/":
            self._serve_static("index.html")
        elif path.startswith("/static/"):
            self._serve_static(path[len("/static/"):])
        elif path == "/api/summary":
            query = urllib.parse.parse_qs(parsed.query)
            self._send_json(self.server.tracker.summary(
                query.get("start", [None])[0],
                query.get("end", [None])[0],
            ))
        elif path == "/api/export":
            self._send_download(
                self.server.tracker.export_csv(),
                "keycounter_stats.csv",
                "text/csv; charset=utf-8",
            )
        elif path == "/api/export/heatmap.png":
            query = urllib.parse.parse_qs(parsed.query)
            day = query.get("date", [date.today().isoformat()])[0]
            data = self.server.tracker.heatmap_png(day)
            if data is None:
                self._send_error(500, "Heatmap image is unavailable")
                return
            self._send_download(data, f"keycounter_heatmap_{day}.png", "image/png")
        elif path == "/api/heatmap":
            query = urllib.parse.parse_qs(parsed.query)
            self._send_json(self.server.tracker.heatmap(
                query.get("start", [None])[0],
                query.get("end", [None])[0],
            ))
        elif path == "/api/trend":
            query = urllib.parse.parse_qs(parsed.query)
            self._send_json(self.server.tracker.trend(
                query.get("start", [None])[0],
                query.get("end", [None])[0],
            ))
        elif path == "/api/settings":
            self._send_json(settings.get_all())
        elif path == "/api/status":
            self._send_json({
                "paused": self.server.tracker.paused,
                "autostart_enabled": autostart.is_enabled(),
            })
        else:
            self._send_error(404, "Not Found")

    def do_POST(self):
        if self.path == "/api/pause":
            length = int(self.headers.get("Content-Length", 0) or 0)
            try:
                body = json.loads(self.rfile.read(length) or b"{}")
            except json.JSONDecodeError:
                body = {}
            paused = bool(body.get("paused", False))
            self.server.tracker.set_paused(paused)
            self._send_json({"paused": self.server.tracker.paused})
        elif self.path == "/api/autostart":
            length = int(self.headers.get("Content-Length", 0) or 0)
            try:
                body = json.loads(self.rfile.read(length) or b"{}")
            except json.JSONDecodeError:
                body = {}
            try:
                if body.get("enabled"):
                    autostart.enable()
                else:
                    autostart.disable()
            except Exception as exc:
                self._send_json({"ok": False, "error": str(exc)})
            else:
                self._send_json({"ok": True, "enabled": autostart.is_enabled()})
        elif self.path == "/api/settings":
            length = int(self.headers.get("Content-Length", 0) or 0)
            try:
                body = json.loads(self.rfile.read(length) or b"{}")
            except json.JSONDecodeError:
                body = {}
            close_action = body.get("close_action")
            if close_action in ("ask", "minimize", "exit"):
                settings.set("close_action", close_action)
            self._send_json(settings.get_all())
        else:
            self._send_error(404, "Not Found")

    def _serve_static(self, relative_path):
        base = WEB_DIR.resolve()
        try:
            target = (base / relative_path).resolve()
            target.relative_to(base)
        except ValueError:
            self._send_error(403, "Forbidden")
            return

        if target.is_dir():
            target = target / "index.html"
        if not target.is_file():
            self._send_error(404, "Not Found")
            return

        content_type = CONTENT_TYPES.get(target.suffix.lower(), "application/octet-stream")
        self._send_bytes(target.read_bytes(), content_type)

    def _send_json(self, payload):
        data = json.dumps(payload, ensure_ascii=False).encode("utf-8")
        self._send_bytes(data, "application/json; charset=utf-8")

    def _send_error(self, status, message):
        data = message.encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "text/plain; charset=utf-8")
        self.send_header("Content-Length", str(len(data)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(data)

    def _send_download(self, data, filename, content_type):
        self.send_response(200)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(data)))
        self.send_header(
            "Content-Disposition",
            f'attachment; filename="{filename}"',
        )
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(data)

    def _send_bytes(self, data, content_type):
        self.send_response(200)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(data)))
        self.send_header("Cache-Control", "no-store")
        self.send_header("X-Content-Type-Options", "nosniff")
        self.end_headers()
        self.wfile.write(data)

    def log_message(self, format, *args):
        return
