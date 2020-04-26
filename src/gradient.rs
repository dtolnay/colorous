use crate::color::Color;
use std::cmp;

#[derive(Copy, Clone)]
pub struct Gradient {
    pub(crate) eval: &'static dyn EvalGradient,
}

impl Gradient {
    pub fn eval_rational(&self, i: usize, n: usize) -> Color {
        let i = cmp::min(i, n - 1);
        self.eval.eval_rational(i, n)
    }

    pub fn eval_continuous(&self, t: f64) -> Color {
        let t = if t < 0.0 {
            0.0
        } else if t > 1.0 {
            1.0
        } else {
            t
        };
        self.eval.eval_continuous(t)
    }
}

pub(crate) trait EvalGradient {
    fn name(&self) -> &'static str;

    fn eval_rational(&self, i: usize, n: usize) -> Color {
        if n <= 1 {
            self.eval_continuous(1.0)
        } else {
            self.eval_continuous(i as f64 / (n - 1) as f64)
        }
    }

    fn eval_continuous(&self, t: f64) -> Color;
}
