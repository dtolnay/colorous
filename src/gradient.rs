use crate::color::Color;
use std::cmp;
use std::fmt::{self, Debug};

#[derive(Copy, Clone)]
pub struct Gradient {
    pub(crate) eval: &'static dyn EvalGradient,
}

impl Gradient {
    /// Samples the gradient at position `i/n`. Requires `0 ≤ i < n`.
    pub fn eval_rational(&self, i: usize, n: usize) -> Color {
        let i = cmp::min(i, n - 1);
        self.eval.eval_rational(i, n)
    }

    /// Samples the gradient at position `t`. Requires `0.0 ≤ t ≤ 1.0`.
    pub fn eval_continuous(&self, t: f64) -> Color {
        let t = t.max(0.0).min(1.0);
        self.eval.eval_continuous(t)
    }

    pub fn name(&self) -> &'static str {
        self.eval.name()
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
        write!(f, "Gradient({})", self.name())
    }
}
