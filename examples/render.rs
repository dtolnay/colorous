use colorous::*;
use image::ImageBuffer;
use std::process;

const GRADIENTS: [Gradient; 38] = [
    // Sequential (multi-hue)
    TURBO,
    VIRIDIS,
    INFERNO,
    MAGMA,
    PLASMA,
    CIVIDIS,
    WARM,
    COOL,
    CUBEHELIX,
    BLUE_GREEN,
    BLUE_PURPLE,
    GREEN_BLUE,
    ORANGE_RED,
    PURPLE_BLUE_GREEN,
    PURPLE_BLUE,
    PURPLE_RED,
    RED_PURPLE,
    YELLOW_GREEN_BLUE,
    YELLOW_GREEN,
    YELLOW_ORANGE_BROWN,
    YELLOW_ORANGE_RED,
    // Sequential (single-hue)
    BLUES,
    GREENS,
    GREYS,
    ORANGES,
    PURPLES,
    REDS,
    // Diverging
    BROWN_GREEN,
    PURPLE_GREEN,
    PINK_GREEN,
    PURPLE_ORANGE,
    RED_BLUE,
    RED_GREY,
    RED_YELLOW_BLUE,
    RED_YELLOW_GREEN,
    SPECTRAL,
    // Cyclical
    RAINBOW,
    SINEBOW,
];

const CATEGORICALS: [&[Color]; 10] = [
    &CATEGORY10,
    &ACCENT,
    &DARK2,
    &PAIRED,
    &PASTEL1,
    &PASTEL2,
    &SET1,
    &SET2,
    &SET3,
    &TABLEAU10,
];

fn main() {
    let rows = GRADIENTS.len() + CATEGORICALS.len();
    let margin = 1;
    let grid = 40;
    let width = 900;
    let height = rows * grid - margin;
    let mut imgbuf = ImageBuffer::new(width as u32, height as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let (x, y) = (x as usize, y as usize);
        let row = y / grid;
        let col = x / grid;
        let border = y % grid >= grid - margin;
        *pixel = if let Some(gradient) = GRADIENTS.get(row) {
            if border {
                image::Rgb([0, 0, 0])
            } else {
                let Color { r, g, b } = gradient.eval_rational(x, width);
                image::Rgb([r, g, b])
            }
        } else if let Some(scheme) = CATEGORICALS.get(row - GRADIENTS.len()) {
            if col >= scheme.len() {
                let ch = (10 + (x + y) / 10 % 2 * 15) as u8;
                image::Rgb([ch, ch, ch])
            } else if border {
                image::Rgb([0, 0, 0])
            } else {
                let Color { r, g, b } = scheme[col];
                image::Rgb([r, g, b])
            }
        } else {
            image::Rgb([0, 0, 0])
        };
    }

    if let Err(err) = imgbuf.save("colorous.png") {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
