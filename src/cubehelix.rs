use crate::Color;
use std::f64::consts as f64;

#[derive(Copy, Clone)]
pub(crate) struct Cubehelix {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

impl From<Cubehelix> for Color {
    fn from(c: Cubehelix) -> Color {
        const DEG2RAD: f64 = f64::PI / 180.0;
        let h = (c.h + 120.0) * DEG2RAD;
        let l = c.l;
        let a = c.s * l * (1.0 - l);
        let cosh = h.cos();
        let sinh = h.sin();
        let r = (255.0 * (l + a * (-0.14861 * cosh + 1.78277 * sinh)).min(1.0)) as u8;
        let g = (255.0 * (l + a * (-0.29227 * cosh - 0.90649 * sinh)).min(1.0)) as u8;
        let b = (255.0 * (l + a * (1.97294 * cosh)).min(1.0)) as u8;
        Color { r, g, b }
    }
}

pub(crate) fn interpolate(start: Cubehelix, end: Cubehelix, t: f64) -> Cubehelix {
    Cubehelix {
        h: start.h + t * (end.h - start.h),
        s: start.s + t * (end.s - start.s),
        l: start.l + t * (end.l - start.l),
    }
}
