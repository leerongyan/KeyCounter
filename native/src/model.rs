//! 采集事件模型与共享工具。

use chrono::Local;

#[derive(Debug, Clone)]
pub enum Event {
    Key { name: String, at: String },
    Click { button: String, x: i32, y: i32, at: String },
    Scroll { button: String, x: i32, y: i32, at: String },
    Move { x: i32, y: i32, distance: f64, at: String },
}

pub fn now_text() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 日期范围解析，与 Python tracker._range_bounds 一致。
/// 返回 (起始日标签, 结束日标签, SQL 起始, SQL 结束)。
pub fn range_bounds(start: Option<&str>, end: Option<&str>) -> (String, String, String, String) {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let all = matches!(start, Some("all")) || matches!(end, Some("all"));
    if all {
        return (
            "全部".to_string(),
            "全部".to_string(),
            "0000-01-01 00:00:00".to_string(),
            "9999-12-31 23:59:59".to_string(),
        );
    }
    let valid = |s: &str| {
        let bytes = s.as_bytes();
        bytes.len() == 10
            && bytes[4] == b'-'
            && bytes[7] == b'-'
            && s[..4].bytes().all(|b| b.is_ascii_digit())
            && s[5..7].bytes().all(|b| b.is_ascii_digit())
            && s[8..10].bytes().all(|b| b.is_ascii_digit())
            && {
                let (y, m, d) = (s[..4].parse::<i32>().unwrap_or(0), s[5..7].parse::<u32>().unwrap_or(0), s[8..10].parse::<u32>().unwrap_or(0));
                (1..=12).contains(&m) && (1..=31).contains(&d) && y > 0
            }
    };
    let mut s = match start {
        Some(v) if valid(v) => v.to_string(),
        _ => today.clone(),
    };
    let mut e = match end {
        Some(v) if valid(v) => v.to_string(),
        _ => today.clone(),
    };
    if s > e {
        std::mem::swap(&mut s, &mut e);
    }
    (
        s.clone(),
        e.clone(),
        format!("{s} 00:00:00"),
        format!("{e} 23:59:59"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_range() {
        let (ls, le, ss, se) = range_bounds(Some("all"), None);
        assert_eq!(ls, "全部");
        assert_eq!(ss, "0000-01-01 00:00:00");
        assert_eq!(se, "9999-12-31 23:59:59");
        assert_eq!(le, "全部");
    }

    #[test]
    fn swap_when_reversed() {
        let (ls, le, ss, se) = range_bounds(Some("2026-08-03"), Some("2026-08-01"));
        assert_eq!(ls, "2026-08-01");
        assert_eq!(le, "2026-08-03");
        assert_eq!(ss, "2026-08-01 00:00:00");
        assert_eq!(se, "2026-08-03 23:59:59");
    }

    #[test]
    fn invalid_falls_back_to_today() {
        // Python 行为：无效 start 回退今天，随后与更早的 end 交换
        let (ls, le, ss, se) = range_bounds(Some("bad"), Some("2026-08-01"));
        let today = Local::now().format("%Y-%m-%d").to_string();
        assert_eq!(ls, "2026-08-01");
        assert_eq!(le, today);
        assert_eq!(ss, "2026-08-01 00:00:00");
        assert_eq!(se, format!("{today} 23:59:59"));
    }
}
