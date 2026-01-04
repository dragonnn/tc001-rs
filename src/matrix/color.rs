use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor as _};

pub fn darken(color: Rgb888, factor: u8) -> Rgb888 {
    // Convert RGB to HSV (integer approximation)
    let r = color.r() as u16;
    let g = color.g() as u16;
    let b = color.b() as u16;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    // Value (V) is max component
    let v = max;

    // Saturation (scaled 0–255)
    let s = if max == 0 { 0 } else { (delta * 255) / max };

    // Hue calculation (0–255 range)
    let h = if delta == 0 {
        0
    } else if max == r {
        (((g + 6 * delta - b) * 43) / delta) % 256
    } else if max == g {
        (((b + 2 * delta - r) * 43) / delta) % 256
    } else {
        (((r + 4 * delta - g) * 43) / delta) % 256
    };

    // Darken by scaling V
    let v = (v * factor as u16) / 255;

    hsv_to_rgb(h as u8, s as u8, v as u8)
}

fn hsv_to_rgb(h: u8, s: u8, v: u8) -> Rgb888 {
    if s == 0 {
        return Rgb888::new(v, v, v);
    }

    let region = h / 43;
    let remainder = (h - region * 43) * 6;

    let p = (v as u16 * (255 - s as u16)) >> 8;
    let q = (v as u16 * (255 - ((s as u16 * remainder as u16) >> 8))) >> 8;
    let t = (v as u16 * (255 - ((s as u16 * (255 - remainder as u16)) >> 8))) >> 8;

    match region {
        0 => Rgb888::new(v, t as u8, p as u8),
        1 => Rgb888::new(q as u8, v, p as u8),
        2 => Rgb888::new(p as u8, v, t as u8),
        3 => Rgb888::new(p as u8, q as u8, v),
        4 => Rgb888::new(t as u8, p as u8, v),
        _ => Rgb888::new(v, p as u8, q as u8),
    }
}
