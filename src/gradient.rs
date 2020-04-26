use crate::color::Color;
use std::cmp;
use std::fmt::{self, Debug};

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
        let t = t.max(0.0).min(1.0);
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

impl Debug for Gradient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Gradient({})", self.eval.name())
    }
}
