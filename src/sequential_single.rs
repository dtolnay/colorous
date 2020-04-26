use crate::Color;

#[derive(Copy, Clone)]
pub struct SequentialSingle {
    pub three: [Color; 3],
    pub four: [Color; 4],
    pub five: [Color; 5],
    pub six: [Color; 6],
    pub seven: [Color; 7],
    pub eight: [Color; 8],
    pub nine: [Color; 9],
}

pub const BLUES: SequentialSingle = SequentialSingle {
    three: *colors! {
        b"deebf7",
        b"9ecae1",
        b"3182bd",
    },
    four: *colors! {
        b"eff3ff",
        b"bdd7e7",
        b"6baed6",
        b"2171b5",
    },
    five: *colors! {
        b"eff3ff",
        b"bdd7e7",
        b"6baed6",
        b"3182bd",
        b"08519c",
    },
    six: *colors! {
        b"eff3ff",
        b"c6dbef",
        b"9ecae1",
        b"6baed6",
        b"3182bd",
        b"08519c",
    },
    seven: *colors! {
        b"eff3ff",
        b"c6dbef",
        b"9ecae1",
        b"6baed6",
        b"4292c6",
        b"2171b5",
        b"084594",
    },
    eight: *colors! {
        b"f7fbff",
        b"deebf7",
        b"c6dbef",
        b"9ecae1",
        b"6baed6",
        b"4292c6",
        b"2171b5",
        b"084594",
    },
    nine: *colors! {
        b"f7fbff",
        b"deebf7",
        b"c6dbef",
        b"9ecae1",
        b"6baed6",
        b"4292c6",
        b"2171b5",
        b"08519c",
        b"08306b",
    },
};

pub const GREENS: &[Color; 9] = colors! {
    b"f7fcf5",
    b"e5f5e0",
    b"c7e9c0",
    b"a1d99b",
    b"74c476",
    b"41ab5d",
    b"238b45",
    b"006d2c",
    b"00441b",
};

pub const GREYS: &[Color; 9] = colors! {
    b"ffffff",
    b"f0f0f0",
    b"d9d9d9",
    b"bdbdbd",
    b"969696",
    b"737373",
    b"525252",
    b"252525",
    b"000000",
};

pub const ORANGES: &[Color; 9] = colors! {
    b"fff5eb",
    b"fee6ce",
    b"fdd0a2",
    b"fdae6b",
    b"fd8d3c",
    b"f16913",
    b"d94801",
    b"a63603",
    b"7f2704",
};

pub const PURPLES: &[Color; 9] = colors! {
    b"fcfbfd",
    b"efedf5",
    b"dadaeb",
    b"bcbddc",
    b"9e9ac8",
    b"807dba",
    b"6a51a3",
    b"54278f",
    b"3f007d",
};

pub const REDS: &[Color; 9] = colors! {
    b"fff5f0",
    b"fee0d2",
    b"fcbba1",
    b"fc9272",
    b"fb6a4a",
    b"ef3b2c",
    b"cb181d",
    b"a50f15",
    b"67000d",
};
