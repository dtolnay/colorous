use crate::gradient::EvalGradient;
use crate::{interpolate, Color, Gradient};

pub const TURBO: Gradient = Gradient { eval: &Turbo };

struct Turbo;

impl EvalGradient for Turbo {
    fn name(&self) -> &'static str {
        "Turbo"
    }

    fn eval_continuous(&self, t: f64) -> Color {
        let r = (34.61
            + t * (1172.33 - t * (10793.56 - t * (33300.12 - t * (38394.49 - t * 14825.05)))))
            .max(0.0)
            .min(255.0) as u8;
        let g = (23.31 + t * (557.33 + t * (1225.33 - t * (3574.96 - t * (1073.77 + t * 707.56)))))
            .max(0.0)
            .min(255.0) as u8;
        let b = (27.2
            + t * (3211.1 - t * (15327.97 - t * (27814.0 - t * (22569.18 - t * 6838.66)))))
            .max(0.0)
            .min(255.0) as u8;
        Color { r, g, b }
    }
}

#[derive(Copy, Clone)]
struct SequentialMulti {
    name: &'static str,
    three: [Color; 3],
    four: [Color; 4],
    five: [Color; 5],
    six: [Color; 6],
    seven: [Color; 7],
    eight: [Color; 8],
    nine: [Color; 9],
}

impl EvalGradient for SequentialMulti {
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
            _ => self.eval_continuous(i as f64 / (n - 1) as f64),
        }
    }

    fn eval_continuous(&self, t: f64) -> Color {
        interpolate::spline(&self.nine, t)
    }
}

pub const BLUE_GREEN: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "BuGn",
        three: colors!(3; b"e5f5f9 99d8c9 2ca25f"),
        four: colors!(4; b"edf8fb b2e2e2 66c2a4 238b45"),
        five: colors!(5; b"edf8fb b2e2e2 66c2a4 2ca25f 006d2c"),
        six: colors!(6; b"edf8fb ccece6 99d8c9 66c2a4 2ca25f 006d2c"),
        seven: colors!(7; b"edf8fb ccece6 99d8c9 66c2a4 41ae76 238b45 005824"),
        eight: colors!(8; b"f7fcfd e5f5f9 ccece6 99d8c9 66c2a4 41ae76 238b45 005824"),
        nine: colors!(9; b"f7fcfd e5f5f9 ccece6 99d8c9 66c2a4 41ae76 238b45 006d2c 00441b"),
    },
};

pub const BLUE_PURPLE: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "BuPu",
        three: colors!(3; b"e0ecf4 9ebcda 8856a7"),
        four: colors!(4; b"edf8fb b3cde3 8c96c6 88419d"),
        five: colors!(5; b"edf8fb b3cde3 8c96c6 8856a7 810f7c"),
        six: colors!(6; b"edf8fb bfd3e6 9ebcda 8c96c6 8856a7 810f7c"),
        seven: colors!(7; b"edf8fb bfd3e6 9ebcda 8c96c6 8c6bb1 88419d 6e016b"),
        eight: colors!(8; b"f7fcfd e0ecf4 bfd3e6 9ebcda 8c96c6 8c6bb1 88419d 6e016b"),
        nine: colors!(9; b"f7fcfd e0ecf4 bfd3e6 9ebcda 8c96c6 8c6bb1 88419d 810f7c 4d004b"),
    },
};

pub const GREEN_BLUE: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "GnBu",
        three: colors!(3; b"e0f3db a8ddb5 43a2ca"),
        four: colors!(4; b"f0f9e8 bae4bc 7bccc4 2b8cbe"),
        five: colors!(5; b"f0f9e8 bae4bc 7bccc4 43a2ca 0868ac"),
        six: colors!(6; b"f0f9e8 ccebc5 a8ddb5 7bccc4 43a2ca 0868ac"),
        seven: colors!(7; b"f0f9e8 ccebc5 a8ddb5 7bccc4 4eb3d3 2b8cbe 08589e"),
        eight: colors!(8; b"f7fcf0 e0f3db ccebc5 a8ddb5 7bccc4 4eb3d3 2b8cbe 08589e"),
        nine: colors!(9; b"f7fcf0 e0f3db ccebc5 a8ddb5 7bccc4 4eb3d3 2b8cbe 0868ac 084081"),
    },
};

pub const ORANGE_RED: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "OrRd",
        three: colors!(3; b"fee8c8 fdbb84 e34a33"),
        four: colors!(4; b"fef0d9 fdcc8a fc8d59 d7301f"),
        five: colors!(5; b"fef0d9 fdcc8a fc8d59 e34a33 b30000"),
        six: colors!(6; b"fef0d9 fdd49e fdbb84 fc8d59 e34a33 b30000"),
        seven: colors!(7; b"fef0d9 fdd49e fdbb84 fc8d59 ef6548 d7301f 990000"),
        eight: colors!(8; b"fff7ec fee8c8 fdd49e fdbb84 fc8d59 ef6548 d7301f 990000"),
        nine: colors!(9; b"fff7ec fee8c8 fdd49e fdbb84 fc8d59 ef6548 d7301f b30000 7f0000"),
    },
};

pub const PURPLE_BLUE_GREEN: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "PuBuGn",
        three: colors!(3; b"ece2f0 a6bddb 1c9099"),
        four: colors!(4; b"f6eff7 bdc9e1 67a9cf 02818a"),
        five: colors!(5; b"f6eff7 bdc9e1 67a9cf 1c9099 016c59"),
        six: colors!(6; b"f6eff7 d0d1e6 a6bddb 67a9cf 1c9099 016c59"),
        seven: colors!(7; b"f6eff7 d0d1e6 a6bddb 67a9cf 3690c0 02818a 016450"),
        eight: colors!(8; b"fff7fb ece2f0 d0d1e6 a6bddb 67a9cf 3690c0 02818a 016450"),
        nine: colors!(9; b"fff7fb ece2f0 d0d1e6 a6bddb 67a9cf 3690c0 02818a 016c59 014636"),
    },
};

pub const PURPLE_BLUE: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "PuBu",
        three: colors!(3; b"ece7f2 a6bddb 2b8cbe"),
        four: colors!(4; b"f1eef6 bdc9e1 74a9cf 0570b0"),
        five: colors!(5; b"f1eef6 bdc9e1 74a9cf 2b8cbe 045a8d"),
        six: colors!(6; b"f1eef6 d0d1e6 a6bddb 74a9cf 2b8cbe 045a8d"),
        seven: colors!(7; b"f1eef6 d0d1e6 a6bddb 74a9cf 3690c0 0570b0 034e7b"),
        eight: colors!(8; b"fff7fb ece7f2 d0d1e6 a6bddb 74a9cf 3690c0 0570b0 034e7b"),
        nine: colors!(9; b"fff7fb ece7f2 d0d1e6 a6bddb 74a9cf 3690c0 0570b0 045a8d 023858"),
    },
};

pub const PURPLE_RED: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "PuRd",
        three: colors!(3; b"e7e1ef c994c7 dd1c77"),
        four: colors!(4; b"f1eef6 d7b5d8 df65b0 ce1256"),
        five: colors!(5; b"f1eef6 d7b5d8 df65b0 dd1c77 980043"),
        six: colors!(6; b"f1eef6 d4b9da c994c7 df65b0 dd1c77 980043"),
        seven: colors!(7; b"f1eef6 d4b9da c994c7 df65b0 e7298a ce1256 91003f"),
        eight: colors!(8; b"f7f4f9 e7e1ef d4b9da c994c7 df65b0 e7298a ce1256 91003f"),
        nine: colors!(9; b"f7f4f9 e7e1ef d4b9da c994c7 df65b0 e7298a ce1256 980043 67001f"),
    },
};

pub const RED_PURPLE: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "RdPu",
        three: colors!(3; b"fde0dd fa9fb5 c51b8a"),
        four: colors!(4; b"feebe2 fbb4b9 f768a1 ae017e"),
        five: colors!(5; b"feebe2 fbb4b9 f768a1 c51b8a 7a0177"),
        six: colors!(6; b"feebe2 fcc5c0 fa9fb5 f768a1 c51b8a 7a0177"),
        seven: colors!(7; b"feebe2 fcc5c0 fa9fb5 f768a1 dd3497 ae017e 7a0177"),
        eight: colors!(8; b"fff7f3 fde0dd fcc5c0 fa9fb5 f768a1 dd3497 ae017e 7a0177"),
        nine: colors!(9; b"fff7f3 fde0dd fcc5c0 fa9fb5 f768a1 dd3497 ae017e 7a0177 49006a"),
    },
};

pub const YELLOW_GREEN_BLUE: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "YlGnBu",
        three: colors!(3; b"edf8b1 7fcdbb 2c7fb8"),
        four: colors!(4; b"ffffcc a1dab4 41b6c4 225ea8"),
        five: colors!(5; b"ffffcc a1dab4 41b6c4 2c7fb8 253494"),
        six: colors!(6; b"ffffcc c7e9b4 7fcdbb 41b6c4 2c7fb8 253494"),
        seven: colors!(7; b"ffffcc c7e9b4 7fcdbb 41b6c4 1d91c0 225ea8 0c2c84"),
        eight: colors!(8; b"ffffd9 edf8b1 c7e9b4 7fcdbb 41b6c4 1d91c0 225ea8 0c2c84"),
        nine: colors!(9; b"ffffd9 edf8b1 c7e9b4 7fcdbb 41b6c4 1d91c0 225ea8 253494 081d58"),
    },
};

pub const YELLOW_GREEN: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "YlGn",
        three: colors!(3; b"f7fcb9 addd8e 31a354"),
        four: colors!(4; b"ffffcc c2e699 78c679 238443"),
        five: colors!(5; b"ffffcc c2e699 78c679 31a354 006837"),
        six: colors!(6; b"ffffcc d9f0a3 addd8e 78c679 31a354 006837"),
        seven: colors!(7; b"ffffcc d9f0a3 addd8e 78c679 41ab5d 238443 005a32"),
        eight: colors!(8; b"ffffe5 f7fcb9 d9f0a3 addd8e 78c679 41ab5d 238443 005a32"),
        nine: colors!(9; b"ffffe5 f7fcb9 d9f0a3 addd8e 78c679 41ab5d 238443 006837 004529"),
    },
};

pub const YELLOW_ORANGE_BROWN: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "YlOrBr",
        three: colors!(3; b"fff7bc fec44f d95f0e"),
        four: colors!(4; b"ffffd4 fed98e fe9929 cc4c02"),
        five: colors!(5; b"ffffd4 fed98e fe9929 d95f0e 993404"),
        six: colors!(6; b"ffffd4 fee391 fec44f fe9929 d95f0e 993404"),
        seven: colors!(7; b"ffffd4 fee391 fec44f fe9929 ec7014 cc4c02 8c2d04"),
        eight: colors!(8; b"ffffe5 fff7bc fee391 fec44f fe9929 ec7014 cc4c02 8c2d04"),
        nine: colors!(9; b"ffffe5 fff7bc fee391 fec44f fe9929 ec7014 cc4c02 993404 662506"),
    },
};

pub const YELLOW_ORANGE_RED: Gradient = Gradient {
    eval: &SequentialMulti {
        name: "YlOrRd",
        three: colors!(3; b"ffeda0 feb24c f03b20"),
        four: colors!(4; b"ffffb2 fecc5c fd8d3c e31a1c"),
        five: colors!(5; b"ffffb2 fecc5c fd8d3c f03b20 bd0026"),
        six: colors!(6; b"ffffb2 fed976 feb24c fd8d3c f03b20 bd0026"),
        seven: colors!(7; b"ffffb2 fed976 feb24c fd8d3c fc4e2a e31a1c b10026"),
        eight: colors!(8; b"ffffcc ffeda0 fed976 feb24c fd8d3c fc4e2a e31a1c b10026"),
        nine: colors!(9; b"ffffcc ffeda0 fed976 feb24c fd8d3c fc4e2a e31a1c bd0026 800026"),
    },
};
