use crate::gradient::EvalGradient;
use crate::{Color, Gradient};
use std::f64::consts as f64;

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
