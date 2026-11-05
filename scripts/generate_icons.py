#!/usr/bin/env python3
"""Generate the original VYRONIX application icon set.

Design: a bold gradient "V" with a diagnostic pulse line on a dark rounded
square. This is an original VYRONIX product mark — it does not reproduce or
imitate the MERQO corporate logo. No trademark claim is made.
"""
from PIL import Image, ImageDraw
import os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
S = 1024


def lerp(a, b, t):
    return tuple(int(a[i] + (b[i] - a[i]) * t) for i in range(3))


def vertical_gradient(size, top, bottom):
    img = Image.new("RGB", (size, size))
    px = img.load()
    for y in range(size):
        c = lerp(top, bottom, y / (size - 1))
        for x in range(size):
            px[x, y] = c
    return img


def build_icon():
    # Background: dark navy rounded square with a subtle vertical gradient.
    bg = vertical_gradient(S, (13, 20, 38), (7, 11, 22)).convert("RGBA")
    mask = Image.new("L", (S, S), 0)
    md = ImageDraw.Draw(mask)
    md.rounded_rectangle([16, 16, S - 16, S - 16], radius=220, fill=255)
    base = Image.new("RGBA", (S, S), (0, 0, 0, 0))
    base.paste(bg, (0, 0), mask)

    # Subtle inner border glow.
    bd = ImageDraw.Draw(base)
    bd.rounded_rectangle([20, 20, S - 20, S - 20], radius=216, outline=(56, 189, 248, 70), width=6)

    # The "V" glyph as a mask.
    v_mask = Image.new("L", (S, S), 0)
    vd = ImageDraw.Draw(v_mask)
    v_poly = [
        (272, 250), (404, 250), (512, 538), (620, 250), (752, 250),
        (576, 806), (448, 806),
    ]
    vd.polygon(v_poly, fill=255)

    # Cyan -> blue gradient for the V.
    v_grad = vertical_gradient(S, (45, 212, 191), (14, 165, 233)).convert("RGBA")
    base.paste(v_grad, (0, 0), v_mask)

    # Diagnostic pulse line across the V (EKG motif), bright cyan-white.
    pd = ImageDraw.Draw(base)
    pulse = [
        (180, 560), (400, 560), (452, 420), (512, 690),
        (568, 480), (612, 560), (844, 560),
    ]
    pd.line(pulse, fill=(224, 250, 255, 255), width=26, joint="curve")
    # Rounded caps.
    for (x, y) in (pulse[0], pulse[-1]):
        pd.ellipse([x - 13, y - 13, x + 13, y + 13], fill=(224, 250, 255, 255))

    return base


def main():
    icon = build_icon()
    tauri_icons = os.path.join(ROOT, "apps", "desktop", "src-tauri", "icons")
    os.makedirs(tauri_icons, exist_ok=True)
    branding = os.path.join(ROOT, "assets", "branding")
    os.makedirs(branding, exist_ok=True)
    assets_icons = os.path.join(ROOT, "assets", "icons")
    os.makedirs(assets_icons, exist_ok=True)

    sizes = {
        "32x32.png": 32,
        "128x128.png": 128,
        "128x128@2x.png": 256,
        "icon.png": 512,
    }
    for name, sz in sizes.items():
        icon.resize((sz, sz), Image.LANCZOS).save(os.path.join(tauri_icons, name))

    # Multi-resolution Windows .ico.
    ico_sizes = [16, 24, 32, 48, 64, 128, 256]
    icon.save(os.path.join(tauri_icons, "icon.ico"), sizes=[(s, s) for s in ico_sizes])

    # Branding copies.
    icon.resize((512, 512), Image.LANCZOS).save(os.path.join(branding, "vyronix-icon-512.png"))
    icon.resize((256, 256), Image.LANCZOS).save(os.path.join(assets_icons, "vyronix-256.png"))
    print("Icons generated:")
    for f in sorted(os.listdir(tauri_icons)):
        print("  apps/desktop/src-tauri/icons/" + f)


if __name__ == "__main__":
    main()
