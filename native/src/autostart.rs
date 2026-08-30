//! 开机自启：注册表 HKCU\Software\Microsoft\Windows\CurrentVersion\Run，值名 KeyCounter。
//! 与 Python 版使用同一个值名（两版不要同时启用自启）。

use winreg::enums::{HKEY_CURRENT_USER, KEY_QUERY_VALUE, KEY_SET_VALUE};
use winreg::RegKey;

const RUN_SUBKEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const VALUE_NAME: &str = "KeyCounter";

fn start_command() -> String {
    let exe = std::env::current_exe().unwrap_or_default();
    format!("\"{}\"", exe.display())
}

pub fn is_enabled() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    match hkcu.open_subkey_with_flags(RUN_SUBKEY, KEY_QUERY_VALUE) {
        Ok(key) => key.get_value::<String, _>(VALUE_NAME).is_ok(),
        Err(_) => false,
    }
}

pub fn enable() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
        .open_subkey_with_flags(RUN_SUBKEY, KEY_SET_VALUE)
        .or_else(|_| hkcu.create_subkey(RUN_SUBKEY).map(|(k, _)| k))
        .expect("open run key");
    key.set_value(VALUE_NAME, &start_command())
        .expect("set run value");
}

pub fn disable() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    match hkcu.open_subkey_with_flags(RUN_SUBKEY, KEY_SET_VALUE) {
        Ok(key) => match key.delete_value(VALUE_NAME) {
            Ok(()) => true,
            Err(_) => false,
        },
        Err(_) => false,
    }
}

pub fn toggle() -> bool {
    if is_enabled() {
        disable();
        false
    } else {
        enable();
        true
    }
}
