#[macro_use]
mod macros;

mod color;
mod interpolate;

pub mod categorical;
pub mod diverging;
pub mod sequential_multi;
pub mod sequential_single;

pub use crate::color::Color;
