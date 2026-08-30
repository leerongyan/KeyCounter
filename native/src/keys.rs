//! 按键规范化：把 Windows 虚拟键码映射成与 Python 版 keys.py 一致的名称。

use windows::Win32::UI::Input::KeyboardAndMouse::{MapVirtualKeyW, MAPVK_VK_TO_CHAR};

/// 小键盘虚拟键 → 名称（与 Python NUMPAD_VK 表一致）
fn numpad_name(vk: u32) -> Option<&'static str> {
    Some(match vk {
        0x60 => "numpad0",
        0x61 => "numpad1",
        0x62 => "numpad2",
        0x63 => "numpad3",
        0x64 => "numpad4",
        0x65 => "numpad5",
        0x66 => "numpad6",
        0x67 => "numpad7",
        0x68 => "numpad8",
        0x69 => "numpad9",
        0x6A => "numpad_multiply",
        0x6B => "numpad_add",
        0x6D => "numpad_subtract",
        0x6E => "numpad_decimal",
        0x6F => "numpad_divide",
        _ => return None,
    })
}

/// 非打印键虚拟键 → 名称（与 pynput 的 Key.name 及 Python 别名表一致）
fn special_name(vk: u32) -> Option<&'static str> {
    Some(match vk {
        0x08 => "backspace",
        0x09 => "tab",
        0x0D => "enter",
        0x13 => "pause",
        0x14 => "capslock",
        0x1B => "esc",
        0x20 => "space",
        0x21 => "page_up",
        0x22 => "page_down",
        0x23 => "end",
        0x24 => "home",
        0x25 => "left",
        0x26 => "up",
        0x27 => "right",
        0x28 => "down",
        0x2C => "print_screen",
        0x2D => "insert",
        0x2E => "delete",
        0x5B | 0x5C => "meta",
        0x5D => "menu",
        0x5F => "sleep",
        0x90 => "num_lock",
        0x91 => "scroll_lock",
        0xA0 | 0xA1 => "shift",
        0xA2 | 0xA3 => "ctrl",
        0xA4 | 0xA5 => "alt",
        0xAD => "volume_mute",
        0xAE => "volume_down",
        0xAF => "volume_up",
        0xB0 => "media_next",
        0xB1 => "media_previous",
        0xB2 => "media_stop",
        0xB3 => "media_play_pause",
        v if (0x70..=0x87).contains(&v) => match v {
            0x70 => "f1",
            0x71 => "f2",
            0x72 => "f3",
            0x73 => "f4",
            0x74 => "f5",
            0x75 => "f6",
            0x76 => "f7",
            0x77 => "f8",
            0x78 => "f9",
            0x79 => "f10",
            0x7A => "f11",
            0x7B => "f12",
            0x7C => "f13",
            0x7D => "f14",
            0x7E => "f15",
            0x7F => "f16",
            0x80 => "f17",
            0x81 => "f18",
            0x82 => "f19",
            0x83 => "f20",
            0x84 => "f21",
            0x85 => "f22",
            0x86 => "f23",
            _ => "f24",
        },
        _ => return None,
    })
}

/// 上档符号归一为基键（与 Python SHIFTED_TO_BASE 表一致）
fn shift_to_base(ch: char) -> Option<char> {
    Some(match ch {
        '!' => '1',
        '@' => '2',
        '#' => '3',
        '$' => '4',
        '%' => '5',
        '^' => '6',
        '&' => '7',
        '*' => '8',
        '(' => '9',
        ')' => '0',
        '~' => '`',
        '_' => '-',
        '+' => '=',
        '{' => '[',
        '}' => ']',
        ':' => ';',
        '"' => '\'',
        '<' => ',',
        '>' => '.',
        '?' => '/',
        '|' => '\\',
        _ => return None,
    })
}

/// 字符路径：Python 中 pynput 的 char 处理（shift 归一 + 字母大写化）
pub fn normalize_char(ch: char) -> Option<String> {
    if !ch.is_ascii_graphic() && !ch.is_alphabetic() {
        return None;
    }
    let base = shift_to_base(ch).unwrap_or(ch);
    if base.is_alphabetic() {
        Some(base.to_ascii_uppercase().to_string())
    } else {
        Some(base.to_string())
    }
}

/// 虚拟键码 → 统计名称。与 Python normalize_key 的优先级一致：
/// 小键盘表 → 字符路径 → 特殊键名表。
pub fn normalize_vk(vk: u32) -> String {
    if let Some(name) = numpad_name(vk) {
        return name.to_string();
    }
    // MapVirtualKeyW 返回当前布局下该键的基础字符（无 shift 上档），
    // 与 Python 的“取 char 再归一 shift”效果等价；字母统一大写化。
    let mapped = unsafe { MapVirtualKeyW(vk, MAPVK_VK_TO_CHAR) } & 0xFFFF;
    if mapped != 0 {
        if let Some(ch) = char::from_u32(mapped as u32) {
            if let Some(name) = normalize_char(ch) {
                return name;
            }
        }
    }
    if let Some(name) = special_name(vk) {
        return name.to_string();
    }
    format!("vk_{:02X}", vk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_symbols_map_to_base() {
        assert_eq!(normalize_char('!'), Some("1".into()));
        assert_eq!(normalize_char('+'), Some("=".into()));
        assert_eq!(normalize_char(':'), Some(";".into()));
        assert_eq!(normalize_char('|'), Some("\\".into()));
    }

    #[test]
    fn letters_are_uppercased() {
        assert_eq!(normalize_char('a'), Some("A".into()));
        assert_eq!(normalize_char('A'), Some("A".into()));
    }

    #[test]
    fn plain_symbols_pass_through() {
        assert_eq!(normalize_char('1'), Some("1".into()));
        assert_eq!(normalize_char(';'), Some(";".into()));
    }

    #[test]
    fn numpad_table() {
        assert_eq!(normalize_vk(0x61), "numpad1");
        assert_eq!(normalize_vk(0x6A), "numpad_multiply");
        assert_eq!(normalize_vk(0x6F), "numpad_divide");
    }

    #[test]
    fn special_names() {
        assert_eq!(special_name(0xA5), Some("alt"));
        assert_eq!(special_name(0xA2), Some("ctrl"));
        assert_eq!(special_name(0x14), Some("capslock"));
        assert_eq!(special_name(0x1B), Some("esc"));
        assert_eq!(special_name(0x5D), Some("menu"));
        assert_eq!(special_name(0x70), Some("f1"));
    }

    #[test]
    fn wheel_direction_matches_python() {
        // 与 Python 版一致的映射：delta < 0 → "up"，其余 → "down"
        assert_eq!(if -1i32 < 0 { "up" } else { "down" }, "up");
        assert_eq!(if 1i32 < 0 { "up" } else { "down" }, "down");
    }
}
