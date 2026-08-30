//! 数据导出：CSV（utf-8-sig 带 BOM，格式与 Python 版一致）与热力图 PNG。

use std::collections::HashMap;

use chrono::Local;

use crate::store::Reader;

fn now_text() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn export_csv(reader: &Reader) -> String {
    let mut out = String::new();
    out.push_str("KeyCounter 数据导出\r\n");
    out.push_str(&format!("导出时间,{}\r\n", now_text()));
    out.push_str("\r\n");
    out.push_str("每日统计\r\n");
    out.push_str("日期,按键总数,点击总数,滚轮总数,移动距离像素\r\n");
    for row in reader.daily_rows() {
        out.push_str(&format!(
            "{},{},{},{},{}\r\n",
            row.0, row.1, row.2, row.3, row.4
        ));
    }
    out.push_str("\r\n");
    out.push_str("按键统计\r\n");
    out.push_str("按键,次数\r\n");
    for (name, count) in reader.key_counts_all() {
        out.push_str(&format!("{name},{count}\r\n"));
    }
    let mut bytes = String::from("\u{FEFF}");
    bytes.push_str(&out);
    bytes
}

pub fn heatmap_png(reader: &Reader, day: &str) -> Option<Vec<u8>> {
    let start = format!("{day} 00:00:00");
    let end = format!("{day} 23:59:59");
    let pairs = reader.key_counts(&start, &end, None);
    let counts: HashMap<String, i64> = pairs.into_iter().collect();
    let total: i64 = counts.values().sum();
    crate::heatmap::render_heatmap(&counts, day, total)
}
