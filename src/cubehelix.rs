#![allow(clippy::many_single_char_names)]

use crate::math::{cos, sin};
use crate::Color;
use core::f32::consts as f32;

#[derive(Copy, Clone)]
pub(crate) struct Cubehelix {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

impl From<Cubehelix> for Color {
    fn from(c: Cubehelix) -> Color {
        const DEG2RAD: f32 = f32::PI / 180.0;
        let h = (c.h + 120.0) * DEG2RAD;
        let l = c.l;
        let a = c.s * l * (1.0 - l);
        let cosh = cos(h);
        let sinh = sin(h);
        let r = (255.0 * (l - a * (0.14861 * cosh - 1.78277 * sinh)).min(1.0)) as u8;
        let g = (255.0 * (l - a * (0.29227 * cosh + 0.90649 * sinh)).min(1.0)) as u8;
        let b = (255.0 * (l + a * (1.97294 * cosh)).min(1.0)) as u8;
        Color { r, g, b }
    }
}

pub(crate) fn interpolate(start: Cubehelix, end: Cubehelix, t: f32) -> Cubehelix {
    Cubehelix {
        h: start.h + t * (end.h - start.h),
        s: start.s + t * (end.s - start.s),
        l: start.l + t * (end.l - start.l),
    }
}
