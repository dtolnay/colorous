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
