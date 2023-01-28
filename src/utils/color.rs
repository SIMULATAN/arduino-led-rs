use crate::Abs;

pub(crate) fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> (u8, u8, u8) {
    let c = value * saturation;
    let x = c * (1.0 - ((hue * 6.0) % 2.0 - 1.0).abs());
    let m = value - c;

    let (r, g, b) = if hue < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if hue < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if hue < 3.0 / 6.0 {
        (0.0, c, x)
    } else if hue < 4.0 / 6.0 {
        (0.0, x, c)
    } else if hue < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}
