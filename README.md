Colorous
========

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/colorous-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/colorous)
[<img alt="crates.io" src="https://img.shields.io/crates/v/colorous.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/colorous)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-colorous-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/colorous)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/colorous/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/colorous/actions?query=branch%3Amaster)

This crate provides a collection of sequential, diverging and categorical color
schemes ported from [d3-scale-chromatic]. Each color scheme may be sampled using
either a rational index `i/n` or a continuous float `0 ≤ t ≤ 1`.

- `pub fn eval_rational(&self, i: usize, n: usize) -> Color`
- `pub fn eval_continuous(&self, t: f64) -> Color`

[d3-scale-chromatic]: https://github.com/d3/d3-scale-chromatic

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/viridis.png" width="100%" height="10" alt="viridis">

## Example

```rust
let gradient = colorous::VIRIDIS;
for i in 0..100 {
    println!("{:x}", gradient.eval_rational(i, 100));
}
```

<br>

## Sequential (multi-hue)

<a href="#Turbo" name="Turbo">#</a> colorous::<b>TURBO</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/turbo.png" width="100%" height="40" alt="turbo">

The “turbo” color scheme by [Anton Mikhailov](https://ai.googleblog.com/2019/08/turbo-improved-rainbow-colormap-for.html).

<a href="#Viridis" name="Viridis">#</a> colorous::<b>VIRIDIS</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/viridis.png" width="100%" height="40" alt="viridis">

The “viridis” perceptually-uniform color scheme designed by [van der Walt, Smith and Firing](https://bids.github.io/colormap/) for matplotlib.

<a href="#Inferno" name="Inferno">#</a> colorous::<b>INFERNO</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/inferno.png" width="100%" height="40" alt="inferno">

The “inferno” perceptually-uniform color scheme designed by [van der Walt and Smith](https://bids.github.io/colormap/) for matplotlib.

<a href="#Magma" name="Magma">#</a> colorous::<b>MAGMA</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/magma.png" width="100%" height="40" alt="magma">

The “magma” perceptually-uniform color scheme designed by [van der Walt and Smith](https://bids.github.io/colormap/) for matplotlib.

<a href="#Plasma" name="Plasma">#</a> colorous::<b>PLASMA</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/plasma.png" width="100%" height="40" alt="plasma">

The “plasma” perceptually-uniform color scheme designed by [van der Walt and Smith](https://bids.github.io/colormap/) for matplotlib.

<a href="#Cividis" name="Cividis">#</a> colorous::<b>CIVIDIS</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/cividis.png" width="100%" height="40" alt="cividis">

The “cividis” color vision deficiency-optimized color scheme designed by [Nuñez, Anderton, and Renslow](https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0199239).

<a href="#Warm" name="Warm">#</a> colorous::<b>WARM</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/warm.png" width="100%" height="40" alt="warm">

A 180° rotation of [Niccoli’s perceptual rainbow](https://mycarta.wordpress.com/2013/02/21/perceptual-rainbow-palette-the-method/).

<a href="#Cool" name="Cool">#</a> colorous::<b>COOL</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/cool.png" width="100%" height="40" alt="cool">

[Niccoli’s perceptual rainbow](https://mycarta.wordpress.com/2013/02/21/perceptual-rainbow-palette-the-method/).

<a href="#CubehelixDefault" name="CubehelixDefault">#</a> colorous::<b>CUBEHELIX</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/cubehelix.png" width="100%" height="40" alt="cubehelix">

[Green’s default Cubehelix](https://www.mrao.cam.ac.uk/~dag/CUBEHELIX/).

<a href="#BuGn" name="BuGn">#</a> colorous::<b>BLUE_GREEN</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/BuGn.png" width="100%" height="40" alt="BuGn">

<a href="#BuPu" name="BuPu">#</a> colorous::<b>BLUE_PURPLE</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/BuPu.png" width="100%" height="40" alt="BuPu">

<a href="#GnBu" name="GnBu">#</a> colorous::<b>GREEN_BLUE</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/GnBu.png" width="100%" height="40" alt="GnBu">

<a href="#OrRd" name="OrRd">#</a> colorous::<b>ORANGE_RED</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/OrRd.png" width="100%" height="40" alt="OrRd">

<a href="#PuBuGn" name="PuBuGn">#</a> colorous::<b>PURPLE_BLUE_GREEN</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/PuBuGn.png" width="100%" height="40" alt="PuBuGn">

<a href="#PuBu" name="PuBu">#</a> colorous::<b>PURPLE_BLUE</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/PuBu.png" width="100%" height="40" alt="PuBu">

<a href="#PuRd" name="PuRd">#</a> colorous::<b>PURPLE_RED</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/PuRd.png" width="100%" height="40" alt="PuRd">

<a href="#RdPu" name="RdPu">#</a> colorous::<b>RED_PURPLE</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/RdPu.png" width="100%" height="40" alt="RdPu">

<a href="#YlGnBu" name="YlGnBu">#</a> colorous::<b>YELLOW_GREEN_BLUE</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/YlGnBu.png" width="100%" height="40" alt="YlGnBu">

<a href="#YlGn" name="YlGn">#</a> colorous::<b>YELLOW_GREEN</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/YlGn.png" width="100%" height="40" alt="YlGn">

<a href="#YlOrBr" name="YlOrBr">#</a> colorous::<b>YELLOW_ORANGE_BROWN</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/YlOrBr.png" width="100%" height="40" alt="YlOrBr">

<a href="#YlOrRd" name="YlOrRd">#</a> colorous::<b>YELLOW_ORANGE_RED</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/YlOrRd.png" width="100%" height="40" alt="YlOrRd">

<br>

## Sequential (single-hue)

<a href="#Blues" name="Blues">#</a> colorous::<b>BLUES</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/Blues.png" width="100%" height="40" alt="Blues">

<a href="#Greens" name="Greens">#</a> colorous::<b>GREENS</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/Greens.png" width="100%" height="40" alt="Greens">

<a href="#Greys" name="Greys">#</a> colorous::<b>GREYS</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/Greys.png" width="100%" height="40" alt="Greys">

<a href="#Oranges" name="Oranges">#</a> colorous::<b>ORANGES</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/Oranges.png" width="100%" height="40" alt="Oranges">

<a href="#Purples" name="Purples">#</a> colorous::<b>PURPLES</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/Purples.png" width="100%" height="40" alt="Purples">

<a href="#Reds" name="Reds">#</a> colorous::<b>REDS</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/Reds.png" width="100%" height="40" alt="Reds">

<br>

## Diverging

<a href="#BrBG" name="BrBG">#</a> colorous::<b>BROWN_GREEN</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/BrBG.png" width="100%" height="40" alt="BrBG">

<a href="#PRGn" name="PRGn">#</a> colorous::<b>PURPLE_GREEN</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/PRGn.png" width="100%" height="40" alt="PRGn">

<a href="#PiYG" name="PiYG">#</a> colorous::<b>PINK_GREEN</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/PiYG.png" width="100%" height="40" alt="PiYG">

<a href="#PuOr" name="PuOr">#</a> colorous::<b>PURPLE_ORANGE</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/PuOr.png" width="100%" height="40" alt="PuOr">

<a href="#RdBu" name="RdBu">#</a> colorous::<b>RED_BLUE</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/RdBu.png" width="100%" height="40" alt="RdBu">

<a href="#RdGy" name="RdGy">#</a> colorous::<b>RED_GREY</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/RdGy.png" width="100%" height="40" alt="RdGy">

<a href="#RdYlBu" name="RdYlBu">#</a> colorous::<b>RED_YELLOW_BLUE</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/RdYlBu.png" width="100%" height="40" alt="RdYlBu">

<a href="#RdYlGn" name="RdYlGn">#</a> colorous::<b>RED_YELLOW_GREEN</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/RdYlGn.png" width="100%" height="40" alt="RdYlGn">

<a href="#Spectral" name="Spectral">#</a> colorous::<b>SPECTRAL</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/Spectral.png" width="100%" height="40" alt="Spectral">

<br>

## Cyclical

<a href="#Rainbow" name="Rainbow">#</a> colorous::<b>RAINBOW</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/rainbow.png" width="100%" height="40" alt="rainbow">

The cyclical [less-angry rainbow](http://bl.ocks.org/mbostock/310c99e53880faec2434) color scheme.

<a href="#Sinebow" name="Sinebow">#</a> colorous::<b>SINEBOW</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/sinebow.png" width="100%" height="40" alt="sinebow">

The “sinebow” color scheme by [Jim Bumgardner](https://krazydad.com/tutorials/makecolors.php) and [Charlie Loyd](http://basecase.org/env/on-rainbows).

<br>

## Categorical

Categorical color schemes are exposed simply as an array `[Color; N]`.

<a href="#Category10" name="Category10">#</a> colorous::<b>CATEGORY10</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/category10.png" width="100%" height="40" alt="category10">

<a href="#Accent" name="Accent">#</a> colorous::<b>ACCENT</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/Accent.png" width="100%" height="40" alt="Accent">

<a href="#Dark2" name="Dark2">#</a> colorous::<b>DARK2</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/Dark2.png" width="100%" height="40" alt="Dark2">

<a href="#Paired" name="Paired">#</a> colorous::<b>PAIRED</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/Paired.png" width="100%" height="40" alt="Paired">

<a href="#Pastel1" name="Pastel1">#</a> colorous::<b>PASTEL1</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/Pastel1.png" width="100%" height="40" alt="Pastel1">

<a href="#Pastel2" name="Pastel2">#</a> colorous::<b>PASTEL2</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/Pastel2.png" width="100%" height="40" alt="Pastel2">

<a href="#Set1" name="Set1">#</a> colorous::<b>SET1</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/Set1.png" width="100%" height="40" alt="Set1">

<a href="#Set2" name="Set2">#</a> colorous::<b>SET2</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/Set2.png" width="100%" height="40" alt="Set2">

<a href="#Set3" name="Set3">#</a> colorous::<b>SET3</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/Set3.png" width="100%" height="40" alt="Set3">

<a href="#Tableau10" name="Tableau10">#</a> colorous::<b>TABLEAU10</b>

<img src="https://raw.githubusercontent.com/dtolnay/colorous/readme/Tableau10.png" width="100%" height="40" alt="Tableau10">

Ten categorical colors authored by Tableau as part of [Tableau 10](https://www.tableau.com/about/blog/2016/7/colors-upgrade-tableau-10-56782).

<br>

## License

Licensed under the <a href="LICENSE-APACHE">Apache License, Version 2.0</a>.
