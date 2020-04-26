pub(crate) const HEX: [u8; 112] = {
    const __: u8 = 255;
    [
        //   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 0
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 1
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 2
        00, 01, 02, 03, 04, 05, 06, 07, 08, 09, __, __, __, __, __, __, // 3
        __, 10, 11, 12, 13, 14, 15, __, __, __, __, __, __, __, __, __, // 4
        __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 5
        __, 10, 11, 12, 13, 14, 15, __, __, __, __, __, __, __, __, __, // 6
    ]
};

macro_rules! c {
    ($h:ident . $i:literal) => {{
        let o = $i * 7;
        c!([$h[o], $h[o + 1], $h[o + 2], $h[o + 3], $h[o + 4], $h[o + 5]])
    }};
    ($hex:expr) => {{
        use crate::macros::HEX;
        let [rh, rl, gh, gl, bh, bl] = $hex;
        crate::Color {
            r: (HEX[rh as usize] + 1 - 1) * 16 + (HEX[rl as usize] + 1 - 1),
            g: (HEX[gh as usize] + 1 - 1) * 16 + (HEX[gl as usize] + 1 - 1),
            b: (HEX[bh as usize] + 1 - 1) * 16 + (HEX[bl as usize] + 1 - 1),
        }
    }};
}

#[rustfmt::skip]
macro_rules! colors {
    (3; $hex:literal) => {{
        let h = $hex;
        [c!(h.0), c!(h.1), c!(h.2)]
    }};
    (4; $hex:literal) => {{
        let h = $hex;
        [c!(h.0), c!(h.1), c!(h.2), c!(h.3)]
    }};
    (5; $hex:literal) => {{
        let h = $hex;
        [c!(h.0), c!(h.1), c!(h.2), c!(h.3), c!(h.4)]
    }};
    (6; $hex:literal) => {{
        let h = $hex;
        [c!(h.0), c!(h.1), c!(h.2), c!(h.3), c!(h.4), c!(h.5)]
    }};
    (7; $hex:literal) => {{
        let h = $hex;
        [c!(h.0), c!(h.1), c!(h.2), c!(h.3), c!(h.4), c!(h.5), c!(h.6)]
    }};
    (8; $hex:literal) => {{
        let h = $hex;
        [c!(h.0), c!(h.1), c!(h.2), c!(h.3), c!(h.4), c!(h.5), c!(h.6), c!(h.7)]
    }};
    (9; $hex:literal) => {{
        let h = $hex;
        [c!(h.0), c!(h.1), c!(h.2), c!(h.3), c!(h.4), c!(h.5), c!(h.6), c!(h.7), c!(h.8)]
    }};
    (10; $hex:literal) => {{
        let h = $hex;
        [c!(h.0), c!(h.1), c!(h.2), c!(h.3), c!(h.4), c!(h.5), c!(h.6), c!(h.7), c!(h.8), c!(h.9)]
    }};
    (11; $hex:literal) => {{
        let h = $hex;
        [c!(h.0), c!(h.1), c!(h.2), c!(h.3), c!(h.4), c!(h.5), c!(h.6), c!(h.7), c!(h.8), c!(h.9), c!(h.10)]
    }};
    (12; $hex:literal) => {{
        let h = $hex;
        [c!(h.0), c!(h.1), c!(h.2), c!(h.3), c!(h.4), c!(h.5), c!(h.6), c!(h.7), c!(h.8), c!(h.9), c!(h.10), c!(h.11)]
    }};
}
