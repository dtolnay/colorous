use crate::cubehelix::Cubehelix;
use crate::gradient::EvalGradient;
use crate::{Color, Gradient};
use std::f64::consts as f64;

/// <img src="https://raw.githubusercontent.com/dtolnay/essay/img/rainbow.png" width="100%" height="40" alt="rainbow">
pub const RAINBOW: Gradient = Gradient { eval: &Rainbow };

struct Rainbow;

impl EvalGradient for Rainbow {
    fn name(&self) -> &'static str {
        "Rainbow"
    }

    fn eval_continuous(&self, t: f64) -> Color {
        let ts = (t - 0.5).abs();
        let h = 360.0 * t - 100.0;
        let s = 1.5 - 1.5 * ts;
        let l = 0.8 - 0.9 * ts;
        Cubehelix { h, s, l }.into()
    }
}

/// <img src="https://raw.githubusercontent.com/dtolnay/essay/img/sinebow.png" width="100%" height="40" alt="sinebow">
pub const SINEBOW: Gradient = Gradient { eval: &Sinebow };

struct Sinebow;

impl EvalGradient for Sinebow {
    fn name(&self) -> &'static str {
        "Sinebow"
    }

    fn eval_continuous(&self, t: f64) -> Color {
        let t = (0.5 - t) * f64::PI;
        let x = t.sin();
        let r = (255.0 * x * x) as u8;
        let x = (t + f64::FRAC_PI_3).sin();
        let g = (255.0 * x * x) as u8;
        let x = (t + 2.0 * f64::FRAC_PI_3).sin();
        let b = (255.0 * x * x) as u8;
        Color { r, g, b }
    }
}
