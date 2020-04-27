#[test]
fn test_continuous() {
    let gradients = vec![
        colorous::BLUES,
        colorous::BLUE_GREEN,
        colorous::BLUE_PURPLE,
        colorous::BROWN_GREEN,
        colorous::CIVIDIS,
        colorous::COOL,
        colorous::CUBEHELIX,
        colorous::GREENS,
        colorous::GREEN_BLUE,
        colorous::GREYS,
        colorous::INFERNO,
        colorous::MAGMA,
        colorous::ORANGES,
        colorous::ORANGE_RED,
        colorous::PINK_GREEN,
        colorous::PLASMA,
        colorous::PURPLES,
        colorous::PURPLE_BLUE,
        colorous::PURPLE_BLUE_GREEN,
        colorous::PURPLE_GREEN,
        colorous::PURPLE_ORANGE,
        colorous::PURPLE_RED,
        colorous::RAINBOW,
        colorous::REDS,
        colorous::RED_BLUE,
        colorous::RED_GREY,
        colorous::RED_PURPLE,
        colorous::RED_YELLOW_BLUE,
        colorous::RED_YELLOW_GREEN,
        colorous::SINEBOW,
        colorous::SPECTRAL,
        colorous::TURBO,
        colorous::VIRIDIS,
        colorous::WARM,
        colorous::YELLOW_GREEN,
        colorous::YELLOW_GREEN_BLUE,
        colorous::YELLOW_ORANGE_BROWN,
        colorous::YELLOW_ORANGE_RED,
    ];

    macro_rules! assert_continuous {
        ($prev:ident, $value:expr) => {
            let value: u8 = $value;
            assert!(((value as i16) - ($prev as i16)).abs() <= 1);
            $prev = value;
        };
    }

    let n = 10000;
    for g in &gradients {
        let first = g.eval_rational(0, n);
        let [mut prev_r, mut prev_g, mut prev_b] = first.as_array();
        for i in 1..n {
            let color = g.eval_rational(i, n);
            assert_continuous!(prev_r, color.r);
            assert_continuous!(prev_g, color.g);
            assert_continuous!(prev_b, color.b);
        }
    }
}
