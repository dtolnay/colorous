use crate::gradient::EvalGradient;
use crate::{interpolate, Color};

#[derive(Copy, Clone)]
pub(crate) struct Sequential {
    pub name: &'static str,
    pub three: [Color; 3],
    pub four: [Color; 4],
    pub five: [Color; 5],
    pub six: [Color; 6],
    pub seven: [Color; 7],
    pub eight: [Color; 8],
    pub nine: [Color; 9],
}

impl EvalGradient for Sequential {
    fn name(&self) -> &'static str {
        self.name
    }

    fn eval_rational(&self, i: usize, n: usize) -> Color {
        match n {
            0 | 1 => self.three[2],
            2 => self.three[i * 2],
            3 => self.three[i],
            4 => self.four[i],
            5 => self.five[i],
            6 => self.six[i],
            7 => self.seven[i],
            8 => self.eight[i],
            9 => self.nine[i],
            _ => self.eval_continuous(i as f32 / (n - 1) as f32),
        }
    }

    fn eval_continuous(&self, t: f32) -> Color {
        interpolate::spline(&self.nine, t)
    }
}
