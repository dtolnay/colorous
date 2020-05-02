//! Floating point arithmetic approximations for `no_std` targets

use core::f32::consts as f32;

pub(crate) trait F32Ext: Sized {
    /// Compute the absolute value of `n`
    fn abs(self) -> f32;

    /// Floor approximation
    fn floor(self) -> f32;

    /// Approximates `cos(x)` in radians with a maximum error of `0.002`
    fn cos(self) -> f32;

    /// Approximates `sin(x)` in radians with a maximum error of `0.002`
    fn sin(self) -> f32;
}


impl F32Ext for f32 {
    fn abs(self) -> f32 {
        f32::from_bits(self.to_bits() & 0x7FFF_FFFF)
    }

    fn floor(self) -> f32 {
        let mut trunc = (self as i32) as f32;

        if self < trunc {
            trunc -= 1.0;
        }

        trunc
    }

    fn cos(self) -> f32 {
        let mut x = self;
        x *= f32::FRAC_1_PI / 2.0;
        x -= 0.25 + (x + 0.25).floor();
        x *= 16.0 * (x.abs() - 0.5);
        x += 0.225 * x * (x.abs() - 1.0);
        x
    }

    fn sin(self) -> f32 {
        (self - f32::PI / 2.0).cos()
    }
}
