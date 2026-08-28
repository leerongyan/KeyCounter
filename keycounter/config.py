from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parent.parent
DATA_DIR = PROJECT_ROOT / "data"
DB_PATH = DATA_DIR / "keycounter.db"
WEB_DIR = PROJECT_ROOT / "web"
DEFAULT_PORT = 8765
