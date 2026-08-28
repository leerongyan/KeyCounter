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

NUMPAD_VK = {
    0x60: "numpad0",
    0x61: "numpad1",
    0x62: "numpad2",
    0x63: "numpad3",
    0x64: "numpad4",
    0x65: "numpad5",
    0x66: "numpad6",
    0x67: "numpad7",
    0x68: "numpad8",
    0x69: "numpad9",
    0x6A: "numpad_multiply",
    0x6B: "numpad_add",
    0x6D: "numpad_subtract",
    0x6E: "numpad_decimal",
    0x6F: "numpad_divide",
}


def normalize_key(key):
    """把 pynput 的按键事件转换为统一按键名称。"""
    numpad_name = NUMPAD_VK.get(getattr(key, "vk", None))
    if numpad_name:
        return numpad_name

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
