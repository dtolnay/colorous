use colorous::*;
use image::ImageBuffer;
use std::process;

const GRADIENTS: [(Gradient, &str); 38] = [
    // Sequential (multi-hue)
    (TURBO, "TURBO"),
    (VIRIDIS, "VIRIDIS"),
    (INFERNO, "INFERNO"),
    (MAGMA, "MAGMA"),
    (PLASMA, "PLASMA"),
    (CIVIDIS, "CIVIDIS"),
    (WARM, "WARM"),
    (COOL, "COOL"),
    (CUBEHELIX, "CUBEHELIX"),
    (BLUE_GREEN, "BLUE_GREEN"),
    (BLUE_PURPLE, "BLUE_PURPLE"),
    (GREEN_BLUE, "GREEN_BLUE"),
    (ORANGE_RED, "ORANGE_RED"),
    (PURPLE_BLUE_GREEN, "PURPLE_BLUE_GREEN"),
    (PURPLE_BLUE, "PURPLE_BLUE"),
    (PURPLE_RED, "PURPLE_RED"),
    (RED_PURPLE, "RED_PURPLE"),
    (YELLOW_GREEN_BLUE, "YELLOW_GREEN_BLUE"),
    (YELLOW_GREEN, "YELLOW_GREEN"),
    (YELLOW_ORANGE_BROWN, "YELLOW_ORANGE_BROWN"),
    (YELLOW_ORANGE_RED, "YELLOW_ORANGE_RED"),
    // Sequential (single-hue)
    (BLUES, "BLUES"),
    (GREENS, "GREENS"),
    (GREYS, "GREYS"),
    (ORANGES, "ORANGES"),
    (PURPLES, "PURPLES"),
    (REDS, "REDS"),
    // Diverging
    (BROWN_GREEN, "BROWN_GREEN"),
    (PURPLE_GREEN, "PURPLE_GREEN"),
    (PINK_GREEN, "PINK_GREEN"),
    (PURPLE_ORANGE, "PURPLE_ORANGE"),
    (RED_BLUE, "RED_BLUE"),
    (RED_GREY, "RED_GREY"),
    (RED_YELLOW_BLUE, "RED_YELLOW_BLUE"),
    (RED_YELLOW_GREEN, "RED_YELLOW_GREEN"),
    (SPECTRAL, "SPECTRAL"),
    // Cyclical
    (RAINBOW, "RAINBOW"),
    (SINEBOW, "SINEBOW"),
];

const CATEGORICALS: [(&[Color], &str); 10] = [
    (&CATEGORY10, "CATEGORY10"),
    (&ACCENT, "ACCENT"),
    (&DARK2, "DARK2"),
    (&PAIRED, "PAIRED"),
    (&PASTEL1, "PASTEL1"),
    (&PASTEL2, "PASTEL2"),
    (&SET1, "SET1"),
    (&SET2, "SET2"),
    (&SET3, "SET3"),
    (&TABLEAU10, "TABLEAU10"),
];

fn main() {
    let rows = GRADIENTS.len() + CATEGORICALS.len();
    let margin = 2;
    let grid = 80;
    let width = 1800;
    let height = rows * grid - margin;
    let mut imgbuf = ImageBuffer::new(width as u32, height as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let (x, y) = (x as usize, y as usize);
        let row = y / grid;
        let col = x / grid;
        let border = y % grid >= grid - margin;
        *pixel = if let Some((gradient, _)) = GRADIENTS.get(row) {
            if border {
                image::Rgb([0, 0, 0])
            } else {
                let i = x.saturating_sub(10);
                let n = width - 20;
                let Color { r, g, b } = gradient.eval_rational(i, n);
                image::Rgb([r, g, b])
            }
        } else if let Some((scheme, _)) = CATEGORICALS.get(row - GRADIENTS.len()) {
            if col >= scheme.len() {
                let ch = ((x + y) / 20 % 2 * 15) as u8 + 10;
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

    let dejavu = dejavu::sans::regular();
    let font = rusttype::Font::try_from_bytes(dejavu).unwrap();

    for row in 0..rows {
        let name = if let Some((_, name)) = GRADIENTS.get(row) {
            name
        } else if let Some((_, name)) = CATEGORICALS.get(row - GRADIENTS.len()) {
            name
        } else {
            continue;
        };
        imageproc::drawing::draw_text_mut(
            &mut imgbuf,
            image::Rgb([100, 100, 100]),
            10,
            (row * grid + 10) as i32,
            rusttype::Scale::uniform(24.0),
            &font,
            name,
        );
    }

    if let Err(err) = imgbuf.save("colorous.png") {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
