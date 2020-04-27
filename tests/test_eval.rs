#[test]
fn test_eval() {
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

    let n = 10000;
    for g in &gradients {
        for i in 0..n {
            let _ = g.eval_rational(i, n);
        }
    }
}
