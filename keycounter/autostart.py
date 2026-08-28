import sys
from pathlib import Path

import winreg

APP_NAME = "KeyCounter"
RUN_KEY = r"Software\Microsoft\Windows\CurrentVersion\Run"


def _start_command():
    if getattr(sys, "frozen", False):
        return f'"{sys.executable}" --no-browser'
    project_root = Path(__file__).resolve().parent.parent
    python_exe = Path(sys.executable)
    pythonw = python_exe.with_name("pythonw.exe")
    launcher = pythonw if pythonw.exists() else python_exe
    return f'"{launcher}" "{project_root / "app.py"}" --no-browser'


def is_enabled():
    try:
        with winreg.OpenKey(winreg.HKEY_CURRENT_USER, RUN_KEY) as key:
            winreg.QueryValueEx(key, APP_NAME)
            return True
    except FileNotFoundError:
        return False


def enable():
    with winreg.CreateKey(winreg.HKEY_CURRENT_USER, RUN_KEY) as key:
        winreg.SetValueEx(key, APP_NAME, 0, winreg.REG_SZ, _start_command())


def disable():
    try:
        with winreg.OpenKey(
            winreg.HKEY_CURRENT_USER, RUN_KEY, 0, winreg.KEY_SET_VALUE
        ) as key:
            winreg.DeleteValue(key, APP_NAME)
    except FileNotFoundError:
        return False
    return True


def toggle():
    if is_enabled():
        disable()
        return False
    enable()
    return True
