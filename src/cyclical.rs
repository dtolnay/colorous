#![allow(clippy::many_single_char_names)]

use crate::cubehelix::Cubehelix;
use crate::gradient::EvalGradient;
use crate::math::{abs, sin};
use crate::{Color, Gradient};
use core::f32::consts as f32;

/// &#8203;
///
/// <img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/rainbow.png" width="100%" height="40" alt="rainbow">
pub const RAINBOW: Gradient = Gradient { eval: &Rainbow };

struct Rainbow;

impl EvalGradient for Rainbow {
    fn name(&self) -> &'static str {
        "Rainbow"
    }

    fn eval_rational(&self, i: usize, n: usize) -> Color {
        eval_rational(self, i, n)
    }

    fn eval_continuous(&self, t: f32) -> Color {
        let ts = abs(t - 0.5);
        let h = 360.0 * t - 100.0;
        let s = 1.5 - 1.5 * ts;
        let l = 0.8 - 0.9 * ts;
        Cubehelix { h, s, l }.into()
    }
}

/// &#8203;
///
/// <img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/sinebow.png" width="100%" height="40" alt="sinebow">
pub const SINEBOW: Gradient = Gradient { eval: &Sinebow };

struct Sinebow;

impl EvalGradient for Sinebow {
    fn name(&self) -> &'static str {
        "Sinebow"
    }

    fn eval_rational(&self, i: usize, n: usize) -> Color {
        eval_rational(self, i, n)
    }

    fn eval_continuous(&self, t: f32) -> Color {
        let t = (0.5 - t) * f32::PI;
        let x = sin(t);
        let r = (255.0 * x * x) as u8;
        let x = sin(t + f32::FRAC_PI_3);
        let g = (255.0 * x * x) as u8;
        let x = sin(t + 2.0 * f32::FRAC_PI_3);
        let b = (255.0 * x * x) as u8;
        Color { r, g, b }
    }
}

// The default implementation in EvalGradient::eval_rational evaluates the
// gradient at `i/(n-1)` to hit both the very first and very last color in the
// gradient. For cyclical gradients the very first and very last color are the
// same, so we want a different behavior.
fn eval_rational(cyclic_gradient: &dyn EvalGradient, i: usize, n: usize) -> Color {
    cyclic_gradient.eval_continuous(i as f32 / n as f32)
}
