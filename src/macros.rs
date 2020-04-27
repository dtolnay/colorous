use crate::Color;

pub(crate) const fn color(n: u32) -> Color {
    Color {
        r: ((n >> 16) & 0xff) as u8,
        g: ((n >> 8) & 0xff) as u8,
        b: (n & 0xff) as u8,
    }
}

macro_rules! colors {
    ($($int:literal)*) => {
        [$(crate::macros::color($int),)*]
    };
}
