use crate::color::Color;

#[derive(Copy, Clone)]
pub struct Gradient {
    pub(crate) eval: &'static dyn EvalGradient,
}

impl Gradient {
    pub fn eval_continuous(&self, t: f64) -> Color {
        self.eval.eval_continuous(t)
    }
}

pub(crate) trait EvalGradient {
    fn eval_continuous(&self, t: f64) -> Color;
}
