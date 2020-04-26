use crate::Color;

#[derive(Copy, Clone)]
pub struct Diverging {
    pub three: [Color; 3],
    pub four: [Color; 4],
    pub five: [Color; 5],
    pub six: [Color; 6],
    pub seven: [Color; 7],
    pub eight: [Color; 8],
    pub nine: [Color; 9],
    pub ten: [Color; 10],
    pub eleven: [Color; 11],
}

pub const BROWN_GREEN: Diverging = Diverging {
    three: colors!(3; b"d8b365 f5f5f5 5ab4ac"),
    four: colors!(4; b"a6611a dfc27d 80cdc1 018571"),
    five: colors!(5; b"a6611a dfc27d f5f5f5 80cdc1 018571"),
    six: colors!(6; b"8c510a d8b365 f6e8c3 c7eae5 5ab4ac 01665e"),
    seven: colors!(7; b"8c510a d8b365 f6e8c3 f5f5f5 c7eae5 5ab4ac 01665e"),
    eight: colors!(8; b"8c510a bf812d dfc27d f6e8c3 c7eae5 80cdc1 35978f 01665e"),
    nine: colors!(9; b"8c510a bf812d dfc27d f6e8c3 f5f5f5 c7eae5 80cdc1 35978f 01665e"),
    ten: colors!(10; b"543005 8c510a bf812d dfc27d f6e8c3 c7eae5 80cdc1 35978f 01665e 003c30"),
    eleven: colors!(11; b"543005 8c510a bf812d dfc27d f6e8c3 f5f5f5 c7eae5 80cdc1 35978f 01665e 003c30"),
};

pub const PURPLE_GREEN: Diverging = Diverging {
    three: colors!(3; b"af8dc3 f7f7f7 7fbf7b"),
    four: colors!(4; b"7b3294 c2a5cf a6dba0 008837"),
    five: colors!(5; b"7b3294 c2a5cf f7f7f7 a6dba0 008837"),
    six: colors!(6; b"762a83 af8dc3 e7d4e8 d9f0d3 7fbf7b 1b7837"),
    seven: colors!(7; b"762a83 af8dc3 e7d4e8 f7f7f7 d9f0d3 7fbf7b 1b7837"),
    eight: colors!(8; b"762a83 9970ab c2a5cf e7d4e8 d9f0d3 a6dba0 5aae61 1b7837"),
    nine: colors!(9; b"762a83 9970ab c2a5cf e7d4e8 f7f7f7 d9f0d3 a6dba0 5aae61 1b7837"),
    ten: colors!(10; b"40004b 762a83 9970ab c2a5cf e7d4e8 d9f0d3 a6dba0 5aae61 1b7837 00441b"),
    eleven: colors!(11; b"40004b 762a83 9970ab c2a5cf e7d4e8 f7f7f7 d9f0d3 a6dba0 5aae61 1b7837 00441b"),
};

pub const PINK_GREEN: Diverging = Diverging {
    three: colors!(3; b"e9a3c9 f7f7f7 a1d76a"),
    four: colors!(4; b"d01c8b f1b6da b8e186 4dac26"),
    five: colors!(5; b"d01c8b f1b6da f7f7f7 b8e186 4dac26"),
    six: colors!(6; b"c51b7d e9a3c9 fde0ef e6f5d0 a1d76a 4d9221"),
    seven: colors!(7; b"c51b7d e9a3c9 fde0ef f7f7f7 e6f5d0 a1d76a 4d9221"),
    eight: colors!(8; b"c51b7d de77ae f1b6da fde0ef e6f5d0 b8e186 7fbc41 4d9221"),
    nine: colors!(9; b"c51b7d de77ae f1b6da fde0ef f7f7f7 e6f5d0 b8e186 7fbc41 4d9221"),
    ten: colors!(10; b"8e0152 c51b7d de77ae f1b6da fde0ef e6f5d0 b8e186 7fbc41 4d9221 276419"),
    eleven: colors!(11; b"8e0152 c51b7d de77ae f1b6da fde0ef f7f7f7 e6f5d0 b8e186 7fbc41 4d9221 276419"),
};

pub const PURPLE_ORANGE: Diverging = Diverging {
    three: colors!(3; b"998ec3 f7f7f7 f1a340"),
    four: colors!(4; b"5e3c99 b2abd2 fdb863 e66101"),
    five: colors!(5; b"5e3c99 b2abd2 f7f7f7 fdb863 e66101"),
    six: colors!(6; b"542788 998ec3 d8daeb fee0b6 f1a340 b35806"),
    seven: colors!(7; b"542788 998ec3 d8daeb f7f7f7 fee0b6 f1a340 b35806"),
    eight: colors!(8; b"542788 8073ac b2abd2 d8daeb fee0b6 fdb863 e08214 b35806"),
    nine: colors!(9; b"542788 8073ac b2abd2 d8daeb f7f7f7 fee0b6 fdb863 e08214 b35806"),
    ten: colors!(10; b"2d004b 542788 8073ac b2abd2 d8daeb fee0b6 fdb863 e08214 b35806 7f3b08"),
    eleven: colors!(11; b"2d004b 542788 8073ac b2abd2 d8daeb f7f7f7 fee0b6 fdb863 e08214 b35806 7f3b08"),
};

pub const RED_BLUE: Diverging = Diverging {
    three: colors!(3; b"ef8a62 f7f7f7 67a9cf"),
    four: colors!(4; b"ca0020 f4a582 92c5de 0571b0"),
    five: colors!(5; b"ca0020 f4a582 f7f7f7 92c5de 0571b0"),
    six: colors!(6; b"b2182b ef8a62 fddbc7 d1e5f0 67a9cf 2166ac"),
    seven: colors!(7; b"b2182b ef8a62 fddbc7 f7f7f7 d1e5f0 67a9cf 2166ac"),
    eight: colors!(8; b"b2182b d6604d f4a582 fddbc7 d1e5f0 92c5de 4393c3 2166ac"),
    nine: colors!(9; b"b2182b d6604d f4a582 fddbc7 f7f7f7 d1e5f0 92c5de 4393c3 2166ac"),
    ten: colors!(10; b"67001f b2182b d6604d f4a582 fddbc7 d1e5f0 92c5de 4393c3 2166ac 053061"),
    eleven: colors!(11; b"67001f b2182b d6604d f4a582 fddbc7 f7f7f7 d1e5f0 92c5de 4393c3 2166ac 053061"),
};

pub const RED_GRAY: Diverging = Diverging {
    three: colors!(3; b"ef8a62 ffffff 999999"),
    four: colors!(4; b"ca0020 f4a582 bababa 404040"),
    five: colors!(5; b"ca0020 f4a582 ffffff bababa 404040"),
    six: colors!(6; b"b2182b ef8a62 fddbc7 e0e0e0 999999 4d4d4d"),
    seven: colors!(7; b"b2182b ef8a62 fddbc7 ffffff e0e0e0 999999 4d4d4d"),
    eight: colors!(8; b"b2182b d6604d f4a582 fddbc7 e0e0e0 bababa 878787 4d4d4d"),
    nine: colors!(9; b"b2182b d6604d f4a582 fddbc7 ffffff e0e0e0 bababa 878787 4d4d4d"),
    ten: colors!(10; b"67001f b2182b d6604d f4a582 fddbc7 e0e0e0 bababa 878787 4d4d4d 1a1a1a"),
    eleven: colors!(11; b"67001f b2182b d6604d f4a582 fddbc7 ffffff e0e0e0 bababa 878787 4d4d4d 1a1a1a"),
};

pub const RED_YELLOW_BLUE: Diverging = Diverging {
    three: colors!(3; b"fc8d59 ffffbf 91bfdb"),
    four: colors!(4; b"d7191c fdae61 abd9e9 2c7bb6"),
    five: colors!(5; b"d7191c fdae61 ffffbf abd9e9 2c7bb6"),
    six: colors!(6; b"d73027 fc8d59 fee090 e0f3f8 91bfdb 4575b4"),
    seven: colors!(7; b"d73027 fc8d59 fee090 ffffbf e0f3f8 91bfdb 4575b4"),
    eight: colors!(8; b"d73027 f46d43 fdae61 fee090 e0f3f8 abd9e9 74add1 4575b4"),
    nine: colors!(9; b"d73027 f46d43 fdae61 fee090 ffffbf e0f3f8 abd9e9 74add1 4575b4"),
    ten: colors!(10; b"a50026 d73027 f46d43 fdae61 fee090 e0f3f8 abd9e9 74add1 4575b4 313695"),
    eleven: colors!(11; b"a50026 d73027 f46d43 fdae61 fee090 ffffbf e0f3f8 abd9e9 74add1 4575b4 313695"),
};

pub const RED_YELLOW_GREEN: Diverging = Diverging {
    three: colors!(3; b"fc8d59 ffffbf 91cf60"),
    four: colors!(4; b"d7191c fdae61 a6d96a 1a9641"),
    five: colors!(5; b"d7191c fdae61 ffffbf a6d96a 1a9641"),
    six: colors!(6; b"d73027 fc8d59 fee08b d9ef8b 91cf60 1a9850"),
    seven: colors!(7; b"d73027 fc8d59 fee08b ffffbf d9ef8b 91cf60 1a9850"),
    eight: colors!(8; b"d73027 f46d43 fdae61 fee08b d9ef8b a6d96a 66bd63 1a9850"),
    nine: colors!(9; b"d73027 f46d43 fdae61 fee08b ffffbf d9ef8b a6d96a 66bd63 1a9850"),
    ten: colors!(10; b"a50026 d73027 f46d43 fdae61 fee08b d9ef8b a6d96a 66bd63 1a9850 006837"),
    eleven: colors!(11; b"a50026 d73027 f46d43 fdae61 fee08b ffffbf d9ef8b a6d96a 66bd63 1a9850 006837"),
};

pub const SPECTRAL: Diverging = Diverging {
    three: colors!(3; b"fc8d59 ffffbf 99d594"),
    four: colors!(4; b"d7191c fdae61 abdda4 2b83ba"),
    five: colors!(5; b"d7191c fdae61 ffffbf abdda4 2b83ba"),
    six: colors!(6; b"d53e4f fc8d59 fee08b e6f598 99d594 3288bd"),
    seven: colors!(7; b"d53e4f fc8d59 fee08b ffffbf e6f598 99d594 3288bd"),
    eight: colors!(8; b"d53e4f f46d43 fdae61 fee08b e6f598 abdda4 66c2a5 3288bd"),
    nine: colors!(9; b"d53e4f f46d43 fdae61 fee08b ffffbf e6f598 abdda4 66c2a5 3288bd"),
    ten: colors!(10; b"9e0142 d53e4f f46d43 fdae61 fee08b e6f598 abdda4 66c2a5 3288bd 5e4fa2"),
    eleven: colors!(11; b"9e0142 d53e4f f46d43 fdae61 fee08b ffffbf e6f598 abdda4 66c2a5 3288bd 5e4fa2"),
};
