import math
from io import BytesIO

from .layout import KEYBOARD_ROWS, NAV_KEYS, NUMPAD_KEYS

# PIL 体积较大，首次渲染热力图 PNG 时才导入，降低常驻内存
Image = None
ImageDraw = None
ImageFont = None


def _load_pil():
    global Image, ImageDraw, ImageFont
    from PIL import Image as pil_image
    from PIL import ImageDraw as pil_draw
    from PIL import ImageFont as pil_font

    Image, ImageDraw, ImageFont = pil_image, pil_draw, pil_font

KEY_W = 50
KEY_H = 42
GAP = 6
PAD = 24


def _font(size):
    for name in ("segoeui.ttf", "arial.ttf"):
        try:
            return ImageFont.truetype(name, size)
        except OSError:
            continue
    try:
        return ImageFont.load_default(size=size)
    except TypeError:
        return ImageFont.load_default()


def _flat_color(ratio):
    if ratio <= 0:
        return (247, 249, 249)
    return (
        round(247 + (216 - 247) * ratio),
        round(249 + (67 - 249) * ratio),
        round(249 + (47 - 249) * ratio),
    )


def _count_for(key_id, counts):
    if key_id in counts:
        return counts[key_id]
    return counts.get(key_id.upper(), 0)


def render_heatmap(counts, day, total_keys):
    if Image is None:
        _load_pil()
    max_count = max(counts.values(), default=1)
    main_width = int(max(sum(width for _, _, width in row) for row in KEYBOARD_ROWS) * KEY_W + 13 * GAP)
    nav_width = 3 * KEY_W + 2 * GAP
    numpad_width = 4 * KEY_W + 3 * GAP
    keyboard_width = main_width + 14 + nav_width + 14 + numpad_width
    title_height = 68
    legend_height = 34
    canvas_w = keyboard_width + PAD * 2
    canvas_h = PAD + title_height + 6 * KEY_H + 5 * GAP + legend_height + PAD

    image = Image.new("RGBA", (canvas_w, canvas_h), (237, 241, 243, 255))
    draw = ImageDraw.Draw(image)

    draw.text((PAD, PAD), "KeyCounter 键盘热力图", font=_font(26), fill=(22, 33, 29))
    draw.text(
        (PAD, PAD + 36),
        f"{day}  按键总数 {total_keys}",
        font=_font(14),
        fill=(90, 104, 98),
    )

    y = PAD + title_height
    for row in KEYBOARD_ROWS:
        x = PAD
        for key_id, label, width in row:
            cell_w = round(width * KEY_W + (width - 1) * GAP)
            count = _count_for(key_id, counts)
            ratio = math.log1p(count) / math.log1p(max_count) if max_count > 1 and count > 0 else 0
            draw.rounded_rectangle(
                [x, y, x + cell_w, y + KEY_H],
                radius=6,
                fill=_flat_color(ratio),
                outline=(178, 192, 198),
                width=1,
            )
            draw.text(
                (x + cell_w / 2, y + KEY_H / 2),
                label,
                font=_font(12),
                fill=(30, 45, 39),
                anchor="mm",
            )
            x += cell_w + GAP
        y += KEY_H + GAP

    nav_x = PAD + main_width + 14
    nav_y = PAD + title_height
    for key_id, label, row, col in NAV_KEYS:
        x = nav_x + (col - 1) * (KEY_W + GAP)
        yy = nav_y + (row - 1) * (KEY_H + GAP)
        count = _count_for(key_id, counts)
        ratio = math.log1p(count) / math.log1p(max_count) if max_count > 1 and count > 0 else 0
        draw.rounded_rectangle(
            [x, yy, x + KEY_W, yy + KEY_H],
            radius=6,
            fill=_flat_color(ratio),
            outline=(178, 192, 198),
            width=1,
        )
        draw.text(
            (x + KEY_W / 2, yy + KEY_H / 2),
            label,
            font=_font(12),
            fill=(30, 45, 39),
            anchor="mm",
        )

    num_x = nav_x + nav_width + 14
    num_y = PAD + title_height
    for key_id, label, row, col, row_span, col_span in NUMPAD_KEYS:
        cell_w = round(col_span * KEY_W + (col_span - 1) * GAP)
        cell_h = round(row_span * KEY_H + (row_span - 1) * GAP)
        x = num_x + (col - 1) * (KEY_W + GAP)
        yy = num_y + (row - 1) * (KEY_H + GAP)
        count = _count_for(key_id, counts)
        ratio = math.log1p(count) / math.log1p(max_count) if max_count > 1 and count > 0 else 0
        draw.rounded_rectangle(
            [x, yy, x + cell_w, yy + cell_h],
            radius=6,
            fill=_flat_color(ratio),
            outline=(178, 192, 198),
            width=1,
        )
        draw.text(
            (x + cell_w / 2, yy + cell_h / 2),
            label,
            font=_font(12),
            fill=(30, 45, 39),
            anchor="mm",
        )

    legend_y = PAD + title_height + 6 * KEY_H + 5 * GAP + 8
    draw.text((PAD, legend_y), "少", font=_font(12), fill=(90, 104, 98))
    for index in range(21):
        left = PAD + 28 + index * 5
        draw.rectangle(
            [left, legend_y + 2, left + 5, legend_y + 14],
            fill=_flat_color(index / 20),
        )
    draw.text((PAD + 28 + 21 * 5 + 8, legend_y), "多", font=_font(12), fill=(90, 104, 98))

    buffer = BytesIO()
    image.save(buffer, format="PNG")
    return buffer.getvalue()

