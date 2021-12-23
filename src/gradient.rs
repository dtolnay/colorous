use crate::color::Color;
use core::cmp;
use core::fmt::{self, Debug};
#[cfg(not(any(feature = "std", unwind_safe_std_only)))]
use core::panic::{RefUnwindSafe, UnwindSafe};
#[cfg(feature = "std")]
use std::panic::{RefUnwindSafe, UnwindSafe};

#[cfg(any(feature = "std", not(unwind_safe_std_only)))]
type DynEvalGradient = dyn EvalGradient + Send + Sync + UnwindSafe + RefUnwindSafe;
#[cfg(all(not(feature = "std"), unwind_safe_std_only))]
type DynEvalGradient = dyn EvalGradient + Send + Sync;

#[derive(Copy, Clone)]
pub struct Gradient {
    pub(crate) eval: &'static DynEvalGradient,
}

impl Gradient {
    /// Samples the gradient at position `i/n`. Requires `0 ≤ i < n`.
    #[allow(clippy::missing_panics_doc)]
    pub fn eval_rational(&self, i: usize, n: usize) -> Color {
        if n == 0 {
            panic!("invalid argument n=0 in Gradient::eval_rational");
        }
        let i = cmp::min(i, n - 1);
        self.eval.eval_rational(i, n)
    }

    /// Samples the gradient at position `t`. Requires `0.0 ≤ t ≤ 1.0`.
    pub fn eval_continuous(&self, t: f64) -> Color {
        let t = t.max(0.0).min(1.0);
        self.eval.eval_continuous(t as f32)
    }
}

pub(crate) trait EvalGradient {
    fn name(&self) -> &'static str;

    fn eval_rational(&self, i: usize, n: usize) -> Color {
        if n <= 1 {
            self.eval_continuous(1.0)
        } else {
            self.eval_continuous(i as f32 / (n - 1) as f32)
        }
    }

    fn eval_continuous(&self, t: f32) -> Color;
}

impl Debug for Gradient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Gradient({})", self.eval.name())
    }
}
