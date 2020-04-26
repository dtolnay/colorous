use crate::gradient::EvalGradient;
use crate::{interpolate, Color, Gradient};

#[derive(Copy, Clone)]
struct SequentialSingle {
    three: [Color; 3],
    four: [Color; 4],
    five: [Color; 5],
    six: [Color; 6],
    seven: [Color; 7],
    eight: [Color; 8],
    nine: [Color; 9],
}

impl EvalGradient for SequentialSingle {
    fn eval_continuous(&self, t: f64) -> Color {
        interpolate::spline(&self.nine, t)
    }
}

pub const BLUES: Gradient = Gradient {
    eval: &SequentialSingle {
        three: colors!(3; b"deebf7 9ecae1 3182bd"),
        four: colors!(4; b"eff3ff bdd7e7 6baed6 2171b5"),
        five: colors!(5; b"eff3ff bdd7e7 6baed6 3182bd 08519c"),
        six: colors!(6; b"eff3ff c6dbef 9ecae1 6baed6 3182bd 08519c"),
        seven: colors!(7; b"eff3ff c6dbef 9ecae1 6baed6 4292c6 2171b5 084594"),
        eight: colors!(8; b"f7fbff deebf7 c6dbef 9ecae1 6baed6 4292c6 2171b5 084594"),
        nine: colors!(9; b"f7fbff deebf7 c6dbef 9ecae1 6baed6 4292c6 2171b5 08519c 08306b"),
    },
};

pub const GREENS: Gradient = Gradient {
    eval: &SequentialSingle {
        three: colors!(3; b"e5f5e0 a1d99b 31a354"),
        four: colors!(4; b"edf8e9 bae4b3 74c476 238b45"),
        five: colors!(5; b"edf8e9 bae4b3 74c476 31a354 006d2c"),
        six: colors!(6; b"edf8e9 c7e9c0 a1d99b 74c476 31a354 006d2c"),
        seven: colors!(7; b"edf8e9 c7e9c0 a1d99b 74c476 41ab5d 238b45 005a32"),
        eight: colors!(8; b"f7fcf5 e5f5e0 c7e9c0 a1d99b 74c476 41ab5d 238b45 005a32"),
        nine: colors!(9; b"f7fcf5 e5f5e0 c7e9c0 a1d99b 74c476 41ab5d 238b45 006d2c 00441b"),
    },
};

pub const GREYS: Gradient = Gradient {
    eval: &SequentialSingle {
        three: colors!(3; b"f0f0f0 bdbdbd 636363"),
        four: colors!(4; b"f7f7f7 cccccc 969696 525252"),
        five: colors!(5; b"f7f7f7 cccccc 969696 636363 252525"),
        six: colors!(6; b"f7f7f7 d9d9d9 bdbdbd 969696 636363 252525"),
        seven: colors!(7; b"f7f7f7 d9d9d9 bdbdbd 969696 737373 525252 252525"),
        eight: colors!(8; b"ffffff f0f0f0 d9d9d9 bdbdbd 969696 737373 525252 252525"),
        nine: colors!(9; b"ffffff f0f0f0 d9d9d9 bdbdbd 969696 737373 525252 252525 000000"),
    },
};

pub const ORANGES: Gradient = Gradient {
    eval: &SequentialSingle {
        three: colors!(3; b"fee6ce fdae6b e6550d"),
        four: colors!(4; b"feedde fdbe85 fd8d3c d94701"),
        five: colors!(5; b"feedde fdbe85 fd8d3c e6550d a63603"),
        six: colors!(6; b"feedde fdd0a2 fdae6b fd8d3c e6550d a63603"),
        seven: colors!(7; b"feedde fdd0a2 fdae6b fd8d3c f16913 d94801 8c2d04"),
        eight: colors!(8; b"fff5eb fee6ce fdd0a2 fdae6b fd8d3c f16913 d94801 8c2d04"),
        nine: colors!(9; b"fff5eb fee6ce fdd0a2 fdae6b fd8d3c f16913 d94801 a63603 7f2704"),
    },
};

pub const PURPLES: Gradient = Gradient {
    eval: &SequentialSingle {
        three: colors!(3; b"efedf5 bcbddc 756bb1"),
        four: colors!(4; b"f2f0f7 cbc9e2 9e9ac8 6a51a3"),
        five: colors!(5; b"f2f0f7 cbc9e2 9e9ac8 756bb1 54278f"),
        six: colors!(6; b"f2f0f7 dadaeb bcbddc 9e9ac8 756bb1 54278f"),
        seven: colors!(7; b"f2f0f7 dadaeb bcbddc 9e9ac8 807dba 6a51a3 4a1486"),
        eight: colors!(8; b"fcfbfd efedf5 dadaeb bcbddc 9e9ac8 807dba 6a51a3 4a1486"),
        nine: colors!(9; b"fcfbfd efedf5 dadaeb bcbddc 9e9ac8 807dba 6a51a3 54278f 3f007d"),
    },
};

pub const REDS: Gradient = Gradient {
    eval: &SequentialSingle {
        three: colors!(3; b"fee0d2 fc9272 de2d26"),
        four: colors!(4; b"fee5d9 fcae91 fb6a4a cb181d"),
        five: colors!(5; b"fee5d9 fcae91 fb6a4a de2d26 a50f15"),
        six: colors!(6; b"fee5d9 fcbba1 fc9272 fb6a4a de2d26 a50f15"),
        seven: colors!(7; b"fee5d9 fcbba1 fc9272 fb6a4a ef3b2c cb181d 99000d"),
        eight: colors!(8; b"fff5f0 fee0d2 fcbba1 fc9272 fb6a4a ef3b2c cb181d 99000d"),
        nine: colors!(9; b"fff5f0 fee0d2 fcbba1 fc9272 fb6a4a ef3b2c cb181d a50f15 67000d"),
    },
};
