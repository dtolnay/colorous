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

pub const VIRIDIS: Gradient = Gradient {
    eval: &Viridis {
        colors: colors! {
            0x440154 0x440256 0x450457 0x450559 0x46075a 0x46085c 0x460a5d 0x460b5e
            0x470d60 0x470e61 0x471063 0x471164 0x471365 0x481467 0x481668 0x481769
            0x48186a 0x481a6c 0x481b6d 0x481c6e 0x481d6f 0x481f70 0x482071 0x482173
            0x482374 0x482475 0x482576 0x482677 0x482878 0x482979 0x472a7a 0x472c7a
            0x472d7b 0x472e7c 0x472f7d 0x46307e 0x46327e 0x46337f 0x463480 0x453581
            0x453781 0x453882 0x443983 0x443a83 0x443b84 0x433d84 0x433e85 0x423f85
            0x424086 0x424186 0x414287 0x414487 0x404588 0x404688 0x3f4788 0x3f4889
            0x3e4989 0x3e4a89 0x3e4c8a 0x3d4d8a 0x3d4e8a 0x3c4f8a 0x3c508b 0x3b518b
            0x3b528b 0x3a538b 0x3a548c 0x39558c 0x39568c 0x38588c 0x38598c 0x375a8c
            0x375b8d 0x365c8d 0x365d8d 0x355e8d 0x355f8d 0x34608d 0x34618d 0x33628d
            0x33638d 0x32648e 0x32658e 0x31668e 0x31678e 0x31688e 0x30698e 0x306a8e
            0x2f6b8e 0x2f6c8e 0x2e6d8e 0x2e6e8e 0x2e6f8e 0x2d708e 0x2d718e 0x2c718e
            0x2c728e 0x2c738e 0x2b748e 0x2b758e 0x2a768e 0x2a778e 0x2a788e 0x29798e
            0x297a8e 0x297b8e 0x287c8e 0x287d8e 0x277e8e 0x277f8e 0x27808e 0x26818e
            0x26828e 0x26828e 0x25838e 0x25848e 0x25858e 0x24868e 0x24878e 0x23888e
            0x23898e 0x238a8d 0x228b8d 0x228c8d 0x228d8d 0x218e8d 0x218f8d 0x21908d
            0x21918c 0x20928c 0x20928c 0x20938c 0x1f948c 0x1f958b 0x1f968b 0x1f978b
            0x1f988b 0x1f998a 0x1f9a8a 0x1e9b8a 0x1e9c89 0x1e9d89 0x1f9e89 0x1f9f88
            0x1fa088 0x1fa188 0x1fa187 0x1fa287 0x20a386 0x20a486 0x21a585 0x21a685
            0x22a785 0x22a884 0x23a983 0x24aa83 0x25ab82 0x25ac82 0x26ad81 0x27ad81
            0x28ae80 0x29af7f 0x2ab07f 0x2cb17e 0x2db27d 0x2eb37c 0x2fb47c 0x31b57b
            0x32b67a 0x34b679 0x35b779 0x37b878 0x38b977 0x3aba76 0x3bbb75 0x3dbc74
            0x3fbc73 0x40bd72 0x42be71 0x44bf70 0x46c06f 0x48c16e 0x4ac16d 0x4cc26c
            0x4ec36b 0x50c46a 0x52c569 0x54c568 0x56c667 0x58c765 0x5ac864 0x5cc863
            0x5ec962 0x60ca60 0x63cb5f 0x65cb5e 0x67cc5c 0x69cd5b 0x6ccd5a 0x6ece58
            0x70cf57 0x73d056 0x75d054 0x77d153 0x7ad151 0x7cd250 0x7fd34e 0x81d34d
            0x84d44b 0x86d549 0x89d548 0x8bd646 0x8ed645 0x90d743 0x93d741 0x95d840
            0x98d83e 0x9bd93c 0x9dd93b 0xa0da39 0xa2da37 0xa5db36 0xa8db34 0xaadc32
            0xaddc30 0xb0dd2f 0xb2dd2d 0xb5de2b 0xb8de29 0xbade28 0xbddf26 0xc0df25
            0xc2df23 0xc5e021 0xc8e020 0xcae11f 0xcde11d 0xd0e11c 0xd2e21b 0xd5e21a
            0xd8e219 0xdae319 0xdde318 0xdfe318 0xe2e418 0xe5e419 0xe7e419 0xeae51a
            0xece51b 0xefe51c 0xf1e51d 0xf4e61e 0xf6e620 0xf8e621 0xfbe723 0xfde725
        },
    },
};

struct Viridis {
    colors: [Color; 256],
}

impl EvalGradient for Viridis {
    fn name(&self) -> &'static str {
        "Viridis"
    }

    fn eval_continuous(&self, t: f64) -> Color {
        interpolate::spline(&self.colors, t)
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
