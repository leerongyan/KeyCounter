"""按键名称规范化和键盘热力图布局映射。"""

SHIFTED_TO_BASE = {
    "!": "1",
    "@": "2",
    "#": "3",
    "$": "4",
    "%": "5",
    "^": "6",
    "&": "7",
    "*": "8",
    "(": "9",
    ")": "0",
    "~": "`",
    "_": "-",
    "+": "=",
    "{": "[",
    "}": "]",
    ":": ";",
    '"': "'",
    "<": ",",
    ">": ".",
    "?": "/",
    "|": "\\",
}

NAME_ALIASES = {
    "alt_l": "alt",
    "alt_r": "alt",
    "ctrl_l": "ctrl",
    "ctrl_r": "ctrl",
    "shift_r": "shift",
    "cmd": "meta",
    "cmd_r": "meta",
    "command": "meta",
    "caps_lock": "capslock",
}


def normalize_key(key):
    """把 pynput 的按键事件转换为统一按键名称。"""
    char = getattr(key, "char", None)
    if char is not None:
        normalized = SHIFTED_TO_BASE.get(char, char)
        if normalized.isalpha():
            return normalized.upper()
        return normalized

    name = getattr(key, "name", None)
    if not name:
        name = str(key).strip("<>").removeprefix("Key.").strip("\"'")
    return NAME_ALIASES.get(name, name)
