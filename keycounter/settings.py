import json
import threading
from pathlib import Path

from .config import DATA_DIR

SETTINGS_PATH = DATA_DIR / "settings.json"
DEFAULTS = {"close_action": "ask"}
_lock = threading.RLock()


def get_all():
    with _lock:
        if SETTINGS_PATH.exists():
            try:
                data = json.loads(SETTINGS_PATH.read_text(encoding="utf-8"))
                if isinstance(data, dict):
                    return data
            except (OSError, json.JSONDecodeError):
                pass
        return dict(DEFAULTS)


def get(key, default=None):
    return get_all().get(key, default if default is not None else DEFAULTS.get(key))


def set(key, value):
    with _lock:
        data = get_all()
        data[key] = value
        DATA_DIR.mkdir(parents=True, exist_ok=True)
        SETTINGS_PATH.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")
