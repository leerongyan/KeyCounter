import math
import queue
import sys
import threading
import time
from datetime import date, datetime, timedelta

import csv
import io

from pynput import keyboard, mouse

from . import autostart
from .db import Database, now_text
from .heatmap import render_heatmap
from .keys import normalize_key


def _disable_power_throttling():
    """豁免 Windows 后台进程节流。

    玩全屏游戏时本程序处于后台，系统可能降低其 CPU 调度优先级，
    导致键盘钩子回调超时、按键丢失。此处显式关闭对本进程的节流。
    """
    if sys.platform != "win32":
        return
    try:
        import ctypes

        class PowerThrottlingState(ctypes.Structure):
            _fields_ = [
                ("Version", ctypes.c_ulong),
                ("ControlMask", ctypes.c_ulong),
                ("StateMask", ctypes.c_ulong),
            ]

        PROCESS_POWER_THROTTLING_EXECUTION_SPEED = 0x1
        PROCESS_POWER_THROTTLING_CURRENT_VERSION = 1
        PROCESS_POWER_THROTTLING = 4
        state = PowerThrottlingState(
            PROCESS_POWER_THROTTLING_CURRENT_VERSION,
            PROCESS_POWER_THROTTLING_EXECUTION_SPEED,
            0,
        )
        kernel32 = ctypes.windll.kernel32
        kernel32.SetProcessInformation(
            kernel32.GetCurrentProcess(),
            PROCESS_POWER_THROTTLING,
            ctypes.byref(state),
            ctypes.sizeof(state),
        )
    except Exception:
        pass


class Tracker:
    def __init__(self, database):
        self.db = database
        self.paused = False
        self.started_at = datetime.now()
        self._last_move = None
        self._held_keys = set()
        self._keyboard_listener = None
        self._mouse_listener = None
        # 钩子回调只把事件放进内存队列，由独立线程批量写库，
        # 避免在系统钩子里做磁盘 I/O（否则游戏等高负载时按键会丢）。
        self._events = queue.Queue(maxsize=500000)
        self._stop_writer = threading.Event()
        self._writer_thread = None

    def start(self):
        self._start_writer()
        _disable_power_throttling()
        self._keyboard_listener = keyboard.Listener(
            on_press=self._on_key_press,
            on_release=self._on_key_release,
        )
        self._mouse_listener = mouse.Listener(
            on_move=self._on_mouse_move,
            on_click=self._on_mouse_click,
            on_scroll=self._on_mouse_scroll,
        )
        self._keyboard_listener.start()
        self._mouse_listener.start()

    def _start_writer(self):
        if self._writer_thread is None:
            self._writer_thread = threading.Thread(
                target=self._writer_loop,
                name="kc-db-writer",
                daemon=True,
            )
            self._writer_thread.start()

    def _writer_loop(self):
        while not self._stop_writer.is_set():
            batch = []
            try:
                batch.append(self._events.get(timeout=0.4))
                while len(batch) < 500:
                    batch.append(self._events.get_nowait())
            except queue.Empty:
                pass
            if batch:
                self._write_batch(batch)

    def _write_batch(self, batch):
        keys = []
        mouse_events = []
        moves = []
        for kind, payload in batch:
            if kind == "key":
                keys.append(payload)
            elif kind == "mouse":
                mouse_events.append(payload)
            else:
                moves.append(payload)
        try:
            self.db.write_events(keys, mouse_events, moves)
        except Exception:
            pass

    def _enqueue(self, item):
        try:
            self._events.put_nowait(item)
        except queue.Full:
            pass

    def stop(self):
        for listener in (self._keyboard_listener, self._mouse_listener):
            if listener is not None:
                listener.stop()
        self._stop_writer.set()
        if self._writer_thread is not None:
            self._writer_thread.join(timeout=3)
        self._drain_events()
        self.db.close()

    def _drain_events(self):
        batch = []
        while True:
            try:
                batch.append(self._events.get_nowait())
            except queue.Empty:
                break
        if batch:
            self._write_batch(batch)

    def set_paused(self, paused):
        self.paused = bool(paused)
        if not paused:
            self._last_move = None
        self._held_keys = set()

    def _on_key_press(self, key):
        if self.paused:
            return
        try:
            name = normalize_key(key)
        except Exception:
            name = "unknown"
        if name in self._held_keys:
            return
        self._held_keys.add(name)
        self._enqueue(("key", (name, now_text())))

    def _on_key_release(self, key):
        try:
            name = normalize_key(key)
        except Exception:
            return
        self._held_keys.discard(name)

    def _on_mouse_move(self, x, y):
        if self.paused:
            self._last_move = None
            return
        now = time.monotonic()
        last = self._last_move
        if last is not None:
            last_x, last_y, last_time = last
            elapsed = now - last_time
            distance = math.hypot(x - last_x, y - last_y)
            if 0.02 <= elapsed <= 5.0 and distance >= 2.0:
                self._enqueue(("move", (x, y, distance, now_text())))
        self._last_move = (x, y, now)

    def _on_mouse_click(self, x, y, button, pressed):
        if self.paused or not pressed:
            return
        try:
            name = button.name or "unknown"
        except Exception:
            name = "unknown"
        if name in ("unknown", "x1", "x2", "side"):
            self._enqueue(("mouse", ("click", "side", x, y, now_text())))
        else:
            self._enqueue(("mouse", ("click", name, x, y, now_text())))

    def _on_mouse_scroll(self, x, y, dx, dy):
        if self.paused:
            return
        direction = "up" if dy < 0 else "down"
        self._enqueue(("mouse", ("scroll", direction, x, y, now_text())))

    @staticmethod
    def _range_bounds(start, end):
        today = date.today().isoformat()
        if start == "all" or end == "all":
            return "全部", "全部", "0000-01-01 00:00:00", "9999-12-31 23:59:59"
        if not start:
            start = today
        if not end:
            end = today
        try:
            date.fromisoformat(start)
            date.fromisoformat(end)
        except ValueError:
            start = today
            end = today
        if start > end:
            start, end = end, start
        start_text = f"{start} 00:00:00"
        end_text = f"{end} 23:59:59"
        return start, end, start_text, end_text

    def summary(self, start=None, end=None):
        start_day, end_day, start_text, end_text = self._range_bounds(start, end)
        distance_px = self.db.distance(start_text, end_text)
        distance_m = distance_px / 3779.527
        return {
            "start": start_day,
            "end": end_day,
            "date": f"{start_day} 至 {end_day}" if start_day != end_day else start_day,
            "keys": self.db.key_count_range(start_text, end_text),
            "clicks": self.db.mouse_event_count_range(start_text, end_text, "click"),
            "scrolls": self.db.mouse_event_count_range(start_text, end_text, "scroll"),
            "distance_px": round(distance_px, 1),
            "distance_m": round(distance_m, 3),
            "paused": self.paused,
            "autostart_enabled": autostart.is_enabled(),
            "started_at": self.started_at.strftime("%Y-%m-%d %H:%M:%S"),
            "last_updated": now_text(),
            "top_keys": self.db.key_counts(start_text, end_text, limit=15),
            "mouse": self.db.mouse_counts(start_text, end_text),
        }

    def heatmap(self, start=None, end=None):
        start_day, end_day, start_text, end_text = self._range_bounds(start, end)
        return {
            "start": start_day,
            "end": end_day,
            "date": f"{start_day} 至 {end_day}" if start_day != end_day else start_day,
            "keys": self.db.key_counts(start_text, end_text),
        }

    def export_csv(self):
        buffer = io.StringIO()
        writer = csv.writer(buffer)
        writer.writerow(["KeyCounter 数据导出"])
        writer.writerow(["导出时间", now_text()])
        writer.writerow([])
        writer.writerow(["每日统计"])
        writer.writerow(["日期", "按键总数", "点击总数", "滚轮总数", "移动距离像素"])
        for row in self.db.daily_rows():
            writer.writerow([
                row["date"], row["keys"], row["clicks"], row["scrolls"],
                row["distance_px"],
            ])
        writer.writerow([])
        writer.writerow(["按键统计"])
        writer.writerow(["按键", "次数"])
        for row in self.db.key_counts_all():
            writer.writerow([row["key_name"], row["count"]])
        return buffer.getvalue().encode("utf-8-sig")

    def heatmap_png(self, day=None):
        today = date.today().isoformat()
        day = day or today
        try:
            date.fromisoformat(day)
        except ValueError:
            day = today
        start, end = self.db.day_range(day)
        counts = {row["key_name"]: row["count"] for row in self.db.key_counts(start, end)}
        total = sum(counts.values())
        return render_heatmap(counts, day, total)

    def trend(self, start=None, end=None):
        start_day, end_day, start_text, end_text = self._range_bounds(start, end)
        if start_day == end_day and start_day != "全部":
            hours = [{"hour": index, "keys": 0, "clicks": 0, "distance": 0} for index in range(24)]
            for row in self.db.hourly_keys(start_text, end_text):
                hours[int(row["hour"])]["keys"] = row["count"]
            for row in self.db.hourly_mouse_events(start_text, end_text):
                hours[int(row["hour"])]["clicks"] = row["count"]
            for row in self.db.hourly_distance(start_text, end_text):
                hours[int(row["hour"])]["distance"] = round(row["distance"], 1)
            return {"period": "hour", "start": start_day, "end": end_day, "hours": hours}

        by_day = {}
        for row in self.db.daily_keys(start_text, end_text):
            by_day.setdefault(row["day"], {"keys": 0, "clicks": 0, "distance": 0})["keys"] = row["count"]
        for row in self.db.daily_mouse_events(start_text, end_text):
            by_day.setdefault(row["day"], {"keys": 0, "clicks": 0, "distance": 0})["clicks"] = row["count"]
        for row in self.db.daily_distance(start_text, end_text):
            by_day.setdefault(row["day"], {"keys": 0, "clicks": 0, "distance": 0})["distance"] = round(row["distance"], 1)
        days = [{"day": day, **item} for day, item in sorted(by_day.items())]
        return {"period": "day", "start": start_day, "end": end_day, "days": days}
