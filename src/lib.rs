//! # Colorous
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/viridis.png" width="100%" height="20" alt="viridis">
//! <br><br>
//!
//! This crate provides a collection of sequential, diverging and categorical
//! color schemes ported from [d3-scale-chromatic]. Each color scheme may be
//! sampled using either a rational index `i/n` or a continuous float `0 ≤ t ≤
//! 1`.
//!
//! - `pub fn eval_rational(&self, i: usize, n: usize) -> Color`
//! - `pub fn eval_continuous(&self, t: f64) -> Color`
//!
//! [d3-scale-chromatic]: https://github.com/d3/d3-scale-chromatic
//!
//! <br>
//!
//! ## Example
//!
//! ```
//! let gradient = colorous::sequential_multi::VIRIDIS;
//! for i in 0..100 {
//!     println!("{:x}", gradient.eval_rational(i, 100));
//! }
//! ```
//!
//! <br><br>
//!
//! ## Sequential (Multi-Hue)
//!
//! <a href="#Turbo" name="Turbo">#</a> colorous::sequential_multi::<b>TURBO</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/turbo.png" width="100%" height="40" alt="turbo">
//!
//! The “turbo” color scheme by [Anton Mikhailov](https://ai.googleblog.com/2019/08/turbo-improved-rainbow-colormap-for.html).
//!
//! <a href="#Viridis" name="Viridis">#</a> colorous::sequential_multi::<b>VIRIDIS</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/viridis.png" width="100%" height="40" alt="viridis">
//!
//! The “viridis” perceptually-uniform color scheme designed by [van der Walt, Smith and Firing](https://bids.github.io/colormap/) for matplotlib.
//!
//! <a href="#Inferno" name="Inferno">#</a> colorous::sequential_multi::<b>INFERNO</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/inferno.png" width="100%" height="40" alt="inferno">
//!
//! The “inferno” perceptually-uniform color scheme designed by [van der Walt and Smith](https://bids.github.io/colormap/) for matplotlib.
//!
//! <a href="#Magma" name="Magma">#</a> colorous::sequential_multi::<b>MAGMA</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/magma.png" width="100%" height="40" alt="magma">
//!
//! The “magma” perceptually-uniform color scheme designed by [van der Walt and Smith](https://bids.github.io/colormap/) for matplotlib.
//!
//! <a href="#Plasma" name="Plasma">#</a> colorous::sequential_multi::<b>PLASMA</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/plasma.png" width="100%" height="40" alt="plasma">
//!
//! The “plasma” perceptually-uniform color scheme designed by [van der Walt and Smith](https://bids.github.io/colormap/) for matplotlib.
//!
//! <a href="#Cividis" name="Cividis">#</a> colorous::sequential_multi::<b>CIVIDIS</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/cividis.png" width="100%" height="40" alt="cividis">
//!
//! The “cividis” color vision deficiency-optimized color scheme designed by [Nuñez, Anderton, and Renslow](https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0199239).
//!
//! <a href="#Warm" name="Warm">#</a> colorous::sequential_multi::<b>WARM</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/warm.png" width="100%" height="40" alt="warm">
//!
//! A 180° rotation of [Niccoli’s perceptual rainbow](https://mycarta.wordpress.com/2013/02/21/perceptual-rainbow-palette-the-method/).
//!
//! <a href="#Cool" name="Cool">#</a> colorous::sequential_multi::<b>COOL</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/cool.png" width="100%" height="40" alt="cool">
//!
//! [Niccoli’s perceptual rainbow](https://mycarta.wordpress.com/2013/02/21/perceptual-rainbow-palette-the-method/).
//!
//! <a href="#CubehelixDefault" name="CubehelixDefault">#</a> colorous::sequential_multi::<b>CUBEHELIX</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/cubehelix.png" width="100%" height="40" alt="cubehelix">
//!
//! [Green’s default Cubehelix](https://www.mrao.cam.ac.uk/~dag/CUBEHELIX/).
//!
//! <a href="#BuGn" name="BuGn">#</a> colorous::sequential_multi::<b>BLUE_GREEN</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/BuGn.png" width="100%" height="40" alt="BuGn">
//!
//! <a href="#BuPu" name="BuPu">#</a> colorous::sequential_multi::<b>BLUE_PURPLE</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/BuPu.png" width="100%" height="40" alt="BuPu">
//!
//! <a href="#GnBu" name="GnBu">#</a> colorous::sequential_multi::<b>GREEN_BLUE</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/GnBu.png" width="100%" height="40" alt="GnBu">
//!
//! <a href="#OrRd" name="OrRd">#</a> colorous::sequential_multi::<b>ORANGE_RED</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/OrRd.png" width="100%" height="40" alt="OrRd">
//!
//! <a href="#PuBuGn" name="PuBuGn">#</a> colorous::sequential_multi::<b>PURPLE_BLUE_GREEN</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/PuBuGn.png" width="100%" height="40" alt="PuBuGn">
//!
//! <a href="#PuBu" name="PuBu">#</a> colorous::sequential_multi::<b>PURPLE_BLUE</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/PuBu.png" width="100%" height="40" alt="PuBu">
//!
//! <a href="#PuRd" name="PuRd">#</a> colorous::sequential_multi::<b>PURPLE_RED</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/PuRd.png" width="100%" height="40" alt="PuRd">
//!
//! <a href="#RdPu" name="RdPu">#</a> colorous::sequential_multi::<b>RED_PURPLE</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/RdPu.png" width="100%" height="40" alt="RdPu">
//!
//! <a href="#YlGnBu" name="YlGnBu">#</a> colorous::sequential_multi::<b>YELLOW_GREEN_BLUE</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/YlGnBu.png" width="100%" height="40" alt="YlGnBu">
//!
//! <a href="#YlGn" name="YlGn">#</a> colorous::sequential_multi::<b>YELLOW_GREEN</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/YlGn.png" width="100%" height="40" alt="YlGn">
//!
//! <a href="#YlOrBr" name="YlOrBr">#</a> colorous::sequential_multi::<b>YELLOW_ORANGE_BROWN</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/YlOrBr.png" width="100%" height="40" alt="YlOrBr">
//!
//! <a href="#YlOrRd" name="YlOrRd">#</a> colorous::sequential_multi::<b>YELLOW_ORANGE_RED</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/YlOrRd.png" width="100%" height="40" alt="YlOrRd">
//!
//! <br><br>
//!
//! ## Sequential (Single Hue)
//!
//! <a href="#Blues" name="Blues">#</a> colorous::sequential_single::<b>BLUES</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Blues.png" width="100%" height="40" alt="Blues">
//!
//! <a href="#Greens" name="Greens">#</a> colorous::sequential_single::<b>GREENS</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Greens.png" width="100%" height="40" alt="Greens">
//!
//! <a href="#Greys" name="Greys">#</a> colorous::sequential_single::<b>GREYS</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Greys.png" width="100%" height="40" alt="Greys">
//!
//! <a href="#Oranges" name="Oranges">#</a> colorous::sequential_single::<b>ORANGES</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Oranges.png" width="100%" height="40" alt="Oranges">
//!
//! <a href="#Purples" name="Purples">#</a> colorous::sequential_single::<b>PURPLES</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Purples.png" width="100%" height="40" alt="Purples">
//!
//! <a href="#Reds" name="Reds">#</a> colorous::sequential_single::<b>REDS</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Reds.png" width="100%" height="40" alt="Reds">
//!
//! <br><br>
//!
//! ## Diverging
//!
//! <a href="#BrBG" name="BrBG">#</a> colorous::diverging::<b>BROWN_GREEN</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/BrBG.png" width="100%" height="40" alt="BrBG">
//!
//! <a href="#PRGn" name="PRGn">#</a> colorous::diverging::<b>PURPLE_GREEN</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/PRGn.png" width="100%" height="40" alt="PRGn">
//!
//! <a href="#PiYG" name="PiYG">#</a> colorous::diverging::<b>PINK_GREEN</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/PiYG.png" width="100%" height="40" alt="PiYG">
//!
//! <a href="#PuOr" name="PuOr">#</a> colorous::diverging::<b>PURPLE_ORANGE</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/PuOr.png" width="100%" height="40" alt="PuOr">
//!
//! <a href="#RdBu" name="RdBu">#</a> colorous::diverging::<b>RED_BLUE</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/RdBu.png" width="100%" height="40" alt="RdBu">
//!
//! <a href="#RdGy" name="RdGy">#</a> colorous::diverging::<b>RED_GREY</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/RdGy.png" width="100%" height="40" alt="RdGy">
//!
//! <a href="#RdYlBu" name="RdYlBu">#</a> colorous::diverging::<b>RED_YELLOW_BLUE</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/RdYlBu.png" width="100%" height="40" alt="RdYlBu">
//!
//! <a href="#RdYlGn" name="RdYlGn">#</a> colorous::diverging::<b>RED_YELLOW_GREEN</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/RdYlGn.png" width="100%" height="40" alt="RdYlGn">
//!
//! <a href="#Spectral" name="Spectral">#</a> colorous::diverging::<b>SPECTRAL</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Spectral.png" width="100%" height="40" alt="Spectral">
//!
//! <br><br>
//!
//! ## Cyclical
//!
//! <a href="#Rainbow" name="Rainbow">#</a> colorous::cyclical::<b>RAINBOW</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/rainbow.png" width="100%" height="40" alt="rainbow">
//!
//! The cyclical [less-angry rainbow](http://bl.ocks.org/mbostock/310c99e53880faec2434) color scheme.
//!
//! <a href="#Sinebow" name="Sinebow">#</a> colorous::cyclical::<b>SINEBOW</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/sinebow.png" width="100%" height="40" alt="sinebow">
//!
//! The “sinebow” color scheme by [Jim Bumgardner](https://krazydad.com/tutorials/makecolors.php) and [Charlie Loyd](http://basecase.org/env/on-rainbows).
//!
//! <br>
//!
//! ## Categorical
//!
//! Categorical color schemes are represented simply as an array `[Color; N]`.
//!
//! <a href="#Category10" name="Category10">#</a> colorous::categorical::<b>CATEGORY10</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/category10.png" width="100%" height="40" alt="category10">
//!
//! <a href="#Accent" name="Accent">#</a> colorous::categorical::<b>ACCENT</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Accent.png" width="100%" height="40" alt="Accent">
//!
//! <a href="#Dark2" name="Dark2">#</a> colorous::categorical::<b>DARK2</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Dark2.png" width="100%" height="40" alt="Dark2">
//!
//! <a href="#Paired" name="Paired">#</a> colorous::categorical::<b>PAIRED</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Paired.png" width="100%" height="40" alt="Paired">
//!
//! <a href="#Pastel1" name="Pastel1">#</a> colorous::categorical::<b>PASTEL1</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Pastel1.png" width="100%" height="40" alt="Pastel1">
//!
//! <a href="#Pastel2" name="Pastel2">#</a> colorous::categorical::<b>PASTEL2</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Pastel2.png" width="100%" height="40" alt="Pastel2">
//!
//! <a href="#Set1" name="Set1">#</a> colorous::categorical::<b>SET1</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Set1.png" width="100%" height="40" alt="Set1">
//!
//! <a href="#Set2" name="Set2">#</a> colorous::categorical::<b>SET2</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Set2.png" width="100%" height="40" alt="Set2">
//!
//! <a href="#Set3" name="Set3">#</a> colorous::categorical::<b>SET3</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Set3.png" width="100%" height="40" alt="Set3">
//!
//! <a href="#Tableau10" name="Tableau10">#</a> colorous::categorical::<b>TABLEAU10</b>
//!
//! <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Tableau10.png" width="100%" height="40" alt="Tableau10">
//!
//! Ten categorical colors authored by Tableau as part of [Tableau 10](https://www.tableau.com/about/blog/2016/7/colors-upgrade-tableau-10-56782).

#[macro_use]
mod macros;

mod color;
mod cubehelix;
mod gradient;
mod interpolate;

pub mod categorical;
pub mod cyclical;
pub mod diverging;
pub mod sequential_multi;
pub mod sequential_single;

pub use crate::color::Color;
pub use crate::gradient::Gradient;
