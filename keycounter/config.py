import sys
from pathlib import Path

if getattr(sys, "frozen", False):
    PROJECT_ROOT = Path(sys.executable).resolve().parent
    WEB_DIR = Path(getattr(sys, "_MEIPASS", PROJECT_ROOT)) / "web"
else:
    PROJECT_ROOT = Path(__file__).resolve().parent.parent
    WEB_DIR = PROJECT_ROOT / "web"

DATA_DIR = PROJECT_ROOT / "data"
DB_PATH = DATA_DIR / "keycounter.db"
DEFAULT_PORT = 8765
