use crate::color::Color;

#[derive(Copy, Clone)]
pub struct Gradient {
    pub(crate) eval: &'static dyn EvalGradient,
}

impl Gradient {
    pub fn eval_rational(&self, i: usize, n: usize) -> Color {
        self.eval.eval_rational(i, n)
    }

    pub fn eval_continuous(&self, t: f64) -> Color {
        self.eval.eval_continuous(t)
    }
}

pub(crate) trait EvalGradient {
    fn eval_rational(&self, i: usize, n: usize) -> Color {
        if n <= 1 {
            self.eval_continuous(1.0)
        } else {
            self.eval_continuous(i as f64 / (n - 1) as f64)
        }
    }

    fn eval_continuous(&self, t: f64) -> Color;
}
