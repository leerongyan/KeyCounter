//! 热力图 PNG 渲染：视觉规格与 Python 版 heatmap.py 像素级一致。

use std::collections::HashMap;

use ab_glyph::{FontArc, PxScale, PxScaleFont, ScaleFont as _};
use tiny_skia::{FillRule, Paint, Path, PathBuilder, Pixmap, Transform};

const KEY_W: f32 = 50.0;
const KEY_H: f32 = 42.0;
const GAP: f32 = 6.0;
const PAD: f32 = 24.0;

/// 手工构造圆角矩形路径（tiny-skia 无内置 RoundedRect）。
fn round_rect_path(x: f32, y: f32, w: f32, h: f32, r: f32) -> Option<Path> {
    let k = 0.5522847498f32 * r;
    let mut pb = PathBuilder::new();
    pb.move_to(x + r, y);
    pb.line_to(x + w - r, y);
    pb.cubic_to(x + w - r + k, y, x + w, y + r - k, x + w, y + r);
    pb.line_to(x + w, y + h - r);
    pb.cubic_to(x + w, y + h - r + k, x + w - r + k, y + h, x + w - r, y + h);
    pb.line_to(x + r, y + h);
    pb.cubic_to(x + r - k, y + h, x, y + h - r + k, x, y + h - r);
    pb.line_to(x, y + r);
    pb.cubic_to(x, y + r - k, x + r - k, y, x + r, y);
    pb.close();
    pb.finish()
}

type LayoutKey<'a> = (&'a str, &'a str, f32);

pub const KEYBOARD_ROWS: &[&[LayoutKey]] = &[
    &[
        ("esc", "Esc", 1.0), ("f1", "F1", 1.0), ("f2", "F2", 1.0), ("f3", "F3", 1.0),
        ("f4", "F4", 1.0), ("f5", "F5", 1.0), ("f6", "F6", 1.0), ("f7", "F7", 1.0),
        ("f8", "F8", 1.0), ("f9", "F9", 1.0), ("f10", "F10", 1.0), ("f11", "F11", 1.0),
        ("f12", "F12", 1.0),
    ],
    &[
        ("`", "`", 1.0), ("1", "1", 1.0), ("2", "2", 1.0), ("3", "3", 1.0),
        ("4", "4", 1.0), ("5", "5", 1.0), ("6", "6", 1.0), ("7", "7", 1.0),
        ("8", "8", 1.0), ("9", "9", 1.0), ("0", "0", 1.0), ("-", "-", 1.0),
        ("=", "=", 1.0), ("backspace", "Backspace", 2.0),
    ],
    &[
        ("tab", "Tab", 1.5), ("q", "Q", 1.0), ("w", "W", 1.0), ("e", "E", 1.0),
        ("r", "R", 1.0), ("t", "T", 1.0), ("y", "Y", 1.0), ("u", "U", 1.0),
        ("i", "I", 1.0), ("o", "O", 1.0), ("p", "P", 1.0), ("[", "[", 1.0),
        ("]", "]", 1.0), ("\\", "\\", 1.5),
    ],
    &[
        ("capslock", "Caps", 1.75), ("a", "A", 1.0), ("s", "S", 1.0),
        ("d", "D", 1.0), ("f", "F", 1.0), ("g", "G", 1.0), ("h", "H", 1.0),
        ("j", "J", 1.0), ("k", "K", 1.0), ("l", "L", 1.0), (";", ";", 1.0),
        ("'", "'", 1.0), ("enter", "Enter", 2.25),
    ],
    &[
        ("shift", "Shift", 2.25), ("z", "Z", 1.0), ("x", "X", 1.0),
        ("c", "C", 1.0), ("v", "V", 1.0), ("b", "B", 1.0), ("n", "N", 1.0),
        ("m", "M", 1.0), (",", ",", 1.0), (".", ".", 1.0), ("/", "/", 1.0),
        ("shift", "Shift", 2.75),
    ],
    &[
        ("ctrl", "Ctrl", 1.5), ("meta", "Win", 1.25), ("alt", "Alt", 1.25),
        ("space", "Space", 6.25), ("alt_gr", "AltGr", 1.25),
        ("menu", "Menu", 1.25), ("ctrl", "Ctrl", 1.5),
    ],
];

pub const NAV_KEYS: &[(&str, &str, i32, i32)] = &[
    ("insert", "Ins", 1, 1),
    ("home", "Home", 1, 2),
    ("page_up", "PgUp", 1, 3),
    ("delete", "Del", 2, 1),
    ("end", "End", 2, 2),
    ("page_down", "PgDn", 2, 3),
    ("up", "\u{2191}", 4, 2),
    ("left", "\u{2190}", 5, 1),
    ("down", "\u{2193}", 5, 2),
    ("right", "\u{2192}", 5, 3),
];

pub const NUMPAD_KEYS: &[(&str, &str, i32, i32, i32, i32)] = &[
    ("num_lock", "NumLk", 1, 1, 1, 1),
    ("numpad_divide", "/", 1, 2, 1, 1),
    ("numpad_multiply", "*", 1, 3, 1, 1),
    ("numpad_subtract", "-", 1, 4, 1, 1),
    ("numpad7", "7", 2, 1, 1, 1),
    ("numpad8", "8", 2, 2, 1, 1),
    ("numpad9", "9", 2, 3, 1, 1),
    ("numpad_add", "+", 2, 4, 2, 1),
    ("numpad4", "4", 3, 1, 1, 1),
    ("numpad5", "5", 3, 2, 1, 1),
    ("numpad6", "6", 3, 3, 1, 1),
    ("numpad1", "1", 4, 1, 1, 1),
    ("numpad2", "2", 4, 2, 1, 1),
    ("numpad3", "3", 4, 3, 1, 1),
    ("enter", "Enter", 4, 4, 2, 1),
    ("numpad0", "0", 5, 1, 1, 2),
    ("numpad_decimal", ".", 5, 3, 1, 1),
];

fn load_font() -> Option<FontArc> {
    for p in [
        "C:/Windows/Fonts/msyh.ttc",
        "C:/Windows/Fonts/msyhbd.ttc",
        "C:/Windows/Fonts/simhei.ttf",
        "C:/Windows/Fonts/segoeui.ttf",
    ] {
        if let Ok(bytes) = std::fs::read(p) {
            if let Ok(font) = FontArc::try_from_vec(bytes) {
                return Some(font);
            }
        }
    }
    None
}

fn flat_color(ratio: f64) -> [u8; 3] {
    if ratio <= 0.0 {
        return [247, 249, 249];
    }
    [
        (247.0 + (216.0 - 247.0) * ratio).round() as u8,
        (249.0 + (67.0 - 249.0) * ratio).round() as u8,
        (249.0 + (47.0 - 249.0) * ratio).round() as u8,
    ]
}

fn count_for(counts: &HashMap<String, i64>, key_id: &str) -> i64 {
    if let Some(v) = counts.get(key_id) {
        return *v;
    }
    counts.get(&key_id.to_uppercase()).copied().unwrap_or(0)
}

struct Canvas {
    pixmap: Pixmap,
    font: Option<FontArc>,
}

impl Canvas {
    fn fill_round_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [u8; 3]) {
        let Some(path) = round_rect_path(x, y, w, h, 6.0) else {
            return;
        };
        let mut paint = Paint::default();
        paint.set_color_rgba8(color[0], color[1], color[2], 255);
        paint.anti_alias = true;
        self.pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
    }

    /// 1px 描边效果 = 外圈 outline 色矩形 + 内缩 1px 的填充矩形
    fn key_rect(&mut self, x: f32, y: f32, w: f32, h: f32, fill: [u8; 3]) {
        self.fill_round_rect(x, y, w, h, [178, 192, 198]);
        self.fill_round_rect(x + 1.0, y + 1.0, w - 2.0, h - 2.0, fill);
    }

    fn fill_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [u8; 3]) {
        let rect = tiny_skia::Rect::from_xywh(x, y, w, h).unwrap();
        let path = PathBuilder::from_rect(rect);
        let mut paint = Paint::default();
        paint.set_color_rgba8(color[0], color[1], color[2], 255);
        paint.anti_alias = false;
        self.pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
    }

    fn draw_text(&mut self, text: &str, x: f32, y: f32, px: f32, color: [u8; 3], center: bool) {
        use ab_glyph::{Font as _, Glyph as AbGlyph};

        let Some(font) = self.font.as_ref() else { return };
        let scaled = PxScaleFont {
            font: font.clone(),
            scale: PxScale::from(px),
        };
        let scale = scaled.scale;
        let ascent = scaled.ascent();
        let descent = scaled.descent();
        let baseline = if center {
            y + (ascent + descent) / 2.0
        } else {
            y + ascent
        };

        let ids: Vec<ab_glyph::GlyphId> = text.chars().map(|c| scaled.font().glyph_id(c)).collect();
        let mut total = 0.0f32;
        for (i, id) in ids.iter().enumerate() {
            total += scaled.h_advance(*id);
            if let Some(next) = ids.get(i + 1) {
                total += scaled.kern(*id, *next);
            }
        }
        let mut cursor = if center { x - total / 2.0 } else { x };

        for (i, id) in ids.iter().enumerate() {
            let glyph = AbGlyph {
                id: *id,
                scale,
                position: ab_glyph::point(cursor, baseline),
            };
            cursor += scaled.h_advance(*id);
            if let Some(next) = ids.get(i + 1) {
                cursor += scaled.kern(*id, *next);
            }
            if let Some(outlined) = scaled.outline_glyph(glyph) {
                let [r, gc, b] = color;
                outlined.draw(|px, py, v| {
                    if v <= 0.02 {
                        return;
                    }
                    let (xu, yu) = (px as u32, py as u32);
                    if xu >= self.pixmap.width() || yu >= self.pixmap.height() {
                        return;
                    }
                    let idx = ((yu * self.pixmap.width() + xu) * 4) as usize;
                    let buf = self.pixmap.data_mut();
                    let a = v.clamp(0.0, 1.0);
                    buf[idx] = (r as f32 * a + buf[idx] as f32 * (1.0 - a)).round() as u8;
                    buf[idx + 1] = (gc as f32 * a + buf[idx + 1] as f32 * (1.0 - a)).round() as u8;
                    buf[idx + 2] = (b as f32 * a + buf[idx + 2] as f32 * (1.0 - a)).round() as u8;
                    buf[idx + 3] = 255;
                });
            }
        }
    }
}

fn max_row_width() -> f32 {
    KEYBOARD_ROWS
        .iter()
        .map(|row| row.iter().map(|(_, _, w)| w).sum::<f32>())
        .fold(0.0f32, f32::max)
}

/// 渲染指定日期的热力图 PNG（视觉规格与 Python 版一致）。
pub fn render_heatmap(counts: &HashMap<String, i64>, day: &str, total_keys: i64) -> Option<Vec<u8>> {
    let main_width = max_row_width() * KEY_W + 13.0 * GAP; // 828
    let nav_width = 3.0 * KEY_W + 2.0 * GAP; // 162
    let numpad_width = 4.0 * KEY_W + 3.0 * GAP; // 218
    let keyboard_width = main_width + 14.0 + nav_width + 14.0 + numpad_width;
    let title_height = 68.0f32;
    let legend_height = 34.0f32;
    let canvas_w = keyboard_width + PAD * 2.0;
    let canvas_h = PAD + title_height + 6.0 * KEY_H + 5.0 * GAP + legend_height + PAD;

    let mut canvas = Canvas {
        pixmap: Pixmap::new(canvas_w as u32, canvas_h as u32)?,
        font: load_font(),
    };
    canvas.fill_rect(0.0, 0.0, canvas_w, canvas_h, [237, 241, 243]);

    canvas.draw_text("KeyCounter 键盘热力图", PAD, PAD, 26.0, [22, 33, 29], false);
    canvas.draw_text(
        &format!("{day}  按键总数 {total_keys}"),
        PAD,
        PAD + 36.0,
        14.0,
        [90, 104, 98],
        false,
    );

    let max_count = counts.values().copied().max().unwrap_or(1);
    let ratio_of = |count: i64| -> f64 {
        if max_count > 1 && count > 0 {
            (count as f64).ln_1p() / (max_count as f64).ln_1p()
        } else {
            0.0
        }
    };

    let key_y0 = PAD + title_height;
    // 主区
    for (row_index, row) in KEYBOARD_ROWS.iter().enumerate() {
        let mut x = PAD;
        let y = key_y0 + row_index as f32 * (KEY_H + GAP);
        for (key_id, label, width) in row.iter() {
            let cell_w = (width * KEY_W + (width - 1.0) * GAP).round();
            let count = count_for(counts, key_id);
            canvas.key_rect(x, y, cell_w, KEY_H, flat_color(ratio_of(count)));
            canvas.draw_text(label, x + cell_w / 2.0, y + KEY_H / 2.0, 12.0, [30, 45, 39], true);
            x += cell_w + GAP;
        }
    }
    // 导航区
    let nav_x = PAD + main_width + 14.0;
    for (key_id, label, row, col) in NAV_KEYS.iter() {
        let x = nav_x + (*col - 1) as f32 * (KEY_W + GAP);
        let y = key_y0 + (*row - 1) as f32 * (KEY_H + GAP);
        let count = count_for(counts, key_id);
        canvas.key_rect(x, y, KEY_W, KEY_H, flat_color(ratio_of(count)));
        canvas.draw_text(label, x + KEY_W / 2.0, y + KEY_H / 2.0, 12.0, [30, 45, 39], true);
    }
    // 小键盘区
    let num_x = nav_x + nav_width + 14.0;
    for (key_id, label, row, col, row_span, col_span) in NUMPAD_KEYS.iter() {
        let cell_w = (*col_span as f32 * KEY_W + (*col_span - 1) as f32 * GAP).round();
        let cell_h = (*row_span as f32 * KEY_H + (*row_span - 1) as f32 * GAP).round();
        let x = num_x + (*col - 1) as f32 * (KEY_W + GAP);
        let y = key_y0 + (*row - 1) as f32 * (KEY_H + GAP);
        let count = count_for(counts, key_id);
        canvas.key_rect(x, y, cell_w, cell_h, flat_color(ratio_of(count)));
        canvas.draw_text(label, x + cell_w / 2.0, y + cell_h / 2.0, 12.0, [30, 45, 39], true);
    }

    // 图例
    let legend_y = PAD + title_height + 6.0 * KEY_H + 5.0 * GAP + 8.0;
    canvas.draw_text("少", PAD, legend_y, 12.0, [90, 104, 98], false);
    for index in 0..21 {
        let left = PAD + 28.0 + index as f32 * 5.0;
        canvas.fill_rect(left, legend_y + 2.0, 5.0, 12.0, flat_color(index as f64 / 20.0));
    }
    canvas.draw_text("多", PAD + 28.0 + 21.0 * 5.0 + 8.0, legend_y, 12.0, [90, 104, 98], false);

    let pm = canvas.pixmap;
    pm.encode_png().ok()
}

/// 托盘图标：64×64 圆角青色方块 + 白色 KC（与 Python 版一致）。
pub fn tray_icon_rgba() -> Option<(Vec<u8>, u32, u32)> {
    let mut canvas = Canvas {
        pixmap: Pixmap::new(64, 64)?,
        font: load_font(),
    };
    canvas.fill_round_rect(2.0, 2.0, 60.0, 60.0, [15, 118, 110]);
    canvas.draw_text("KC", 32.0, 31.0, 24.0, [255, 255, 255], true);
    let pm = canvas.pixmap;
    Some((pm.data().to_vec(), pm.width(), pm.height()))
}
