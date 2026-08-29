import sqlite3
import threading
from datetime import datetime

from .config import DATA_DIR, DB_PATH


def now_text():
    return datetime.now().strftime("%Y-%m-%d %H:%M:%S")


class Database:
    def __init__(self, path=DB_PATH):
        DATA_DIR.mkdir(parents=True, exist_ok=True)
        self._lock = threading.RLock()
        self.conn = sqlite3.connect(str(path), check_same_thread=False)
        self.conn.row_factory = sqlite3.Row
        with self._lock:
            self.conn.execute("PRAGMA journal_mode=WAL")
            self._init_tables()

    def _init_tables(self):
        self.conn.executescript(
            """
            CREATE TABLE IF NOT EXISTS key_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                key_name TEXT NOT NULL,
                pressed_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS mouse_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                event_type TEXT NOT NULL,
                button TEXT NOT NULL DEFAULT '',
                x INTEGER NOT NULL DEFAULT 0,
                y INTEGER NOT NULL DEFAULT 0,
                pressed_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS mouse_moves (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                x INTEGER NOT NULL,
                y INTEGER NOT NULL,
                distance_px REAL NOT NULL DEFAULT 0,
                sampled_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_key_events_time ON key_events(pressed_at);
            CREATE INDEX IF NOT EXISTS idx_mouse_events_time ON mouse_events(pressed_at);
            CREATE INDEX IF NOT EXISTS idx_mouse_moves_time ON mouse_moves(sampled_at);
            """
        )
        self.conn.commit()

    @staticmethod
    def day_range(day):
        return f"{day} 00:00:00", f"{day} 23:59:59"

    def record_key(self, key_name, when=None):
        with self._lock:
            self.conn.execute(
                "INSERT INTO key_events (key_name, pressed_at) VALUES (?, ?)",
                (key_name, when or now_text()),
            )
            self.conn.commit()

    def record_mouse(self, event_type, button="", x=0, y=0, when=None):
        with self._lock:
            self.conn.execute(
                """
                INSERT INTO mouse_events (event_type, button, x, y, pressed_at)
                VALUES (?, ?, ?, ?, ?)
                """,
                (event_type, button, x, y, when or now_text()),
            )
            self.conn.commit()

    def record_move(self, x, y, distance_px, when=None):
        with self._lock:
            self.conn.execute(
                """
                INSERT INTO mouse_moves (x, y, distance_px, sampled_at)
                VALUES (?, ?, ?, ?)
                """,
                (x, y, distance_px, when or now_text()),
            )
            self.conn.commit()

    def write_events(self, keys=(), mouse_events=(), moves=()):
        """批量写入事件并只提交一次，供采集线程的批量落库使用。"""
        with self._lock:
            if keys:
                self.conn.executemany(
                    "INSERT INTO key_events (key_name, pressed_at) VALUES (?, ?)",
                    keys,
                )
            if mouse_events:
                self.conn.executemany(
                    """
                    INSERT INTO mouse_events (event_type, button, x, y, pressed_at)
                    VALUES (?, ?, ?, ?, ?)
                    """,
                    mouse_events,
                )
            if moves:
                self.conn.executemany(
                    """
                    INSERT INTO mouse_moves (x, y, distance_px, sampled_at)
                    VALUES (?, ?, ?, ?)
                    """,
                    moves,
                )
            if keys or mouse_events or moves:
                self.conn.commit()

    def key_count(self, day):
        start, end = self.day_range(day)
        with self._lock:
            row = self.conn.execute(
                """
                SELECT COUNT(*) AS count FROM key_events
                WHERE pressed_at BETWEEN ? AND ?
                """,
                (start, end),
            ).fetchone()
            return row["count"]

    def key_count_range(self, start, end):
        with self._lock:
            row = self.conn.execute(
                """
                SELECT COUNT(*) AS count FROM key_events
                WHERE pressed_at BETWEEN ? AND ?
                """,
                (start, end),
            ).fetchone()
            return row["count"]

    def mouse_event_count(self, day, event_type):
        start, end = self.day_range(day)
        with self._lock:
            row = self.conn.execute(
                """
                SELECT COUNT(*) AS count FROM mouse_events
                WHERE pressed_at BETWEEN ? AND ? AND event_type = ?
                """,
                (start, end, event_type),
            ).fetchone()
            return row["count"]

    def mouse_event_count_range(self, start, end, event_type):
        with self._lock:
            row = self.conn.execute(
                """
                SELECT COUNT(*) AS count FROM mouse_events
                WHERE pressed_at BETWEEN ? AND ? AND event_type = ?
                """,
                (start, end, event_type),
            ).fetchone()
            return row["count"]

    def key_counts(self, start, end, limit=None):
        sql = """
            SELECT key_name, COUNT(*) AS count FROM key_events
            WHERE pressed_at BETWEEN ? AND ?
            GROUP BY key_name ORDER BY count DESC
        """
        params = [start, end]
        if limit:
            sql += " LIMIT ?"
            params.append(limit)
        with self._lock:
            return [dict(row) for row in self.conn.execute(sql, params).fetchall()]

    def mouse_counts(self, start, end):
        with self._lock:
            rows = self.conn.execute(
                """
                SELECT event_type, button, COUNT(*) AS count FROM mouse_events
                WHERE pressed_at BETWEEN ? AND ?
                GROUP BY event_type, button ORDER BY count DESC
                """,
                (start, end),
            ).fetchall()
            return [dict(row) for row in rows]

    def distance(self, start, end):
        with self._lock:
            row = self.conn.execute(
                """
                SELECT COALESCE(SUM(distance_px), 0) AS distance FROM mouse_moves
                WHERE sampled_at BETWEEN ? AND ?
                """,
                (start, end),
            ).fetchone()
            return row["distance"]

    def hourly_keys(self, start, end):
        with self._lock:
            rows = self.conn.execute(
                """
                SELECT substr(pressed_at, 12, 2) AS hour, COUNT(*) AS count
                FROM key_events
                WHERE pressed_at BETWEEN ? AND ?
                GROUP BY substr(pressed_at, 12, 2)
                """,
                (start, end),
            ).fetchall()
            return [dict(row) for row in rows]

    def hourly_mouse_events(self, start, end):
        with self._lock:
            rows = self.conn.execute(
                """
                SELECT substr(pressed_at, 12, 2) AS hour, COUNT(*) AS count
                FROM mouse_events
                WHERE pressed_at BETWEEN ? AND ?
                GROUP BY substr(pressed_at, 12, 2)
                """,
                (start, end),
            ).fetchall()
            return [dict(row) for row in rows]

    def hourly_distance(self, start, end):
        with self._lock:
            rows = self.conn.execute(
                """
                SELECT substr(sampled_at, 12, 2) AS hour,
                       COALESCE(SUM(distance_px), 0) AS distance
                FROM mouse_moves
                WHERE sampled_at BETWEEN ? AND ?
                GROUP BY substr(sampled_at, 12, 2)
                """,
                (start, end),
            ).fetchall()
            return [dict(row) for row in rows]

    def daily_keys(self, start, end):
        with self._lock:
            rows = self.conn.execute(
                """
                SELECT substr(pressed_at, 1, 10) AS day, COUNT(*) AS count
                FROM key_events
                WHERE pressed_at BETWEEN ? AND ?
                GROUP BY substr(pressed_at, 1, 10)
                ORDER BY day
                """,
                (start, end),
            ).fetchall()
            return [dict(row) for row in rows]

    def daily_mouse_events(self, start, end):
        with self._lock:
            rows = self.conn.execute(
                """
                SELECT substr(pressed_at, 1, 10) AS day, COUNT(*) AS count
                FROM mouse_events
                WHERE pressed_at BETWEEN ? AND ?
                GROUP BY substr(pressed_at, 1, 10)
                ORDER BY day
                """,
                (start, end),
            ).fetchall()
            return [dict(row) for row in rows]

    def daily_distance(self, start, end):
        with self._lock:
            rows = self.conn.execute(
                """
                SELECT substr(sampled_at, 1, 10) AS day,
                       COALESCE(SUM(distance_px), 0) AS distance
                FROM mouse_moves
                WHERE sampled_at BETWEEN ? AND ?
                GROUP BY substr(sampled_at, 1, 10)
                ORDER BY day
                """,
                (start, end),
            ).fetchall()
            return [dict(row) for row in rows]

    def daily_rows(self):
        with self._lock:
            key_rows = self.conn.execute(
                """
                SELECT substr(pressed_at, 1, 10) AS day, COUNT(*) AS count
                FROM key_events
                GROUP BY substr(pressed_at, 1, 10)
                ORDER BY day
                """
            ).fetchall()
            mouse_rows = self.conn.execute(
                """
                SELECT substr(pressed_at, 1, 10) AS day, event_type,
                       COUNT(*) AS count
                FROM mouse_events
                GROUP BY substr(pressed_at, 1, 10), event_type
                ORDER BY day
                """
            ).fetchall()
            distance_rows = self.conn.execute(
                """
                SELECT substr(sampled_at, 1, 10) AS day,
                       COALESCE(SUM(distance_px), 0) AS distance
                FROM mouse_moves
                GROUP BY substr(sampled_at, 1, 10)
                ORDER BY day
                """
            ).fetchall()

        stats = {}
        for row in key_rows:
            stats.setdefault(row["day"], {})["keys"] = row["count"]
        for row in mouse_rows:
            day = row["day"]
            stats.setdefault(day, {})[row["event_type"]] = row["count"]
        for row in distance_rows:
            stats.setdefault(row["day"], {})["distance"] = row["distance"]

        result = []
        for day in sorted(stats):
            item = stats[day]
            result.append({
                "date": day,
                "keys": item.get("keys", 0),
                "clicks": item.get("click", 0),
                "scrolls": item.get("scroll", 0),
                "distance_px": item.get("distance", 0),
            })
        return result

    def key_counts_all(self):
        with self._lock:
            rows = self.conn.execute(
                """
                SELECT key_name, COUNT(*) AS count FROM key_events
                GROUP BY key_name ORDER BY count DESC
                """
            ).fetchall()
            return [dict(row) for row in rows]

    def close(self):
        with self._lock:
            self.conn.close()
