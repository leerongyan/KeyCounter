import math
import time
from datetime import date, datetime, timedelta

from pynput import keyboard, mouse

from .db import Database, now_text
from .keys import normalize_key


class Tracker:
    def __init__(self, database):
        self.db = database
        self.paused = False
        self.started_at = datetime.now()
        self._last_move = None
        self._keyboard_listener = None
        self._mouse_listener = None

    def start(self):
        self._keyboard_listener = keyboard.Listener(on_press=self._on_key_press)
        self._mouse_listener = mouse.Listener(
            on_move=self._on_mouse_move,
            on_click=self._on_mouse_click,
            on_scroll=self._on_mouse_scroll,
        )
        self._keyboard_listener.start()
        self._mouse_listener.start()

    def stop(self):
        for listener in (self._keyboard_listener, self._mouse_listener):
            if listener is not None:
                listener.stop()
        self.db.close()

    def set_paused(self, paused):
        self.paused = bool(paused)
        if not self.paused:
            self._last_move = None

    def _on_key_press(self, key):
        if self.paused:
            return
        try:
            name = normalize_key(key)
        except Exception:
            name = "unknown"
        self.db.record_key(name)

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
                self.db.record_move(x, y, distance)
        self._last_move = (x, y, now)

    def _on_mouse_click(self, x, y, button, pressed):
        if self.paused or not pressed:
            return
        try:
            name = button.name or "unknown"
        except Exception:
            name = "unknown"
        self.db.record_mouse("click", name, x, y)

    def _on_mouse_scroll(self, x, y, dx, dy):
        if self.paused:
            return
        direction = "up" if dy < 0 else "down"
        self.db.record_mouse("scroll", direction, x, y)

    def summary(self):
        today = date.today().isoformat()
        start, end = self.db.day_range(today)
        distance_px = self.db.distance(start, end)
        distance_m = distance_px / 3779.527
        return {
            "date": today,
            "keys": self.db.key_count(today),
            "clicks": self.db.mouse_event_count(today, "click"),
            "scrolls": self.db.mouse_event_count(today, "scroll"),
            "distance_px": round(distance_px, 1),
            "distance_m": round(distance_m, 3),
            "paused": self.paused,
            "started_at": self.started_at.strftime("%Y-%m-%d %H:%M:%S"),
            "last_updated": now_text(),
            "top_keys": self.db.key_counts(start, end, limit=10),
            "mouse": self.db.mouse_counts(start, end),
        }

    def heatmap(self, day=None):
        today = date.today().isoformat()
        day = day or today
        try:
            date.fromisoformat(day)
        except ValueError:
            day = today
        start, end = self.db.day_range(day)
        return {"date": day, "keys": self.db.key_counts(start, end)}

    def trend(self):
        today = date.today().isoformat()
        start, end = self.db.day_range(today)
        hours = [{"hour": index, "keys": 0, "clicks": 0, "distance": 0} for index in range(24)]
        for row in self.db.hourly_keys(start, end):
            hours[int(row["hour"])]["keys"] = row["count"]
        for row in self.db.hourly_mouse_events(start, end):
            hours[int(row["hour"])]["clicks"] = row["count"]
        for row in self.db.hourly_distance(start, end):
            hours[int(row["hour"])]["distance"] = round(row["distance"], 1)
        return {"date": today, "hours": hours}
