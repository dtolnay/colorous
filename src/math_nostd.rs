// Floating point arithmetic approximations for `no_std` targets. Vendored from:
// https://github.com/NeoBirth/micromath/tree/e82158853632661e6ba411f145d222d9deb8babb

use core::f32::consts as f32;

pub fn abs(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7FFF_FFFF)
}

pub fn floor(x: f32) -> f32 {
    let mut trunc = (x as i32) as f32;

    if x < trunc {
        trunc -= 1.0;
    }

    trunc
}

pub fn cos(mut x: f32) -> f32 {
    x *= f32::FRAC_1_PI / 2.0;
    x -= 0.25 + floor(x + 0.25);
    x *= 16.0 * (abs(x) - 0.5);
    x += 0.225 * x * (abs(x) - 1.0);
    x
}

pub fn sin(x: f32) -> f32 {
    cos(x - f32::PI / 2.0)
}
