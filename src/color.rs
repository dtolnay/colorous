use std::fmt::{self, Debug, LowerHex, UpperHex};

/// &#8203;
///
/// `Color` has public fields `r`, `g`, `b` of type `u8` which you can read
/// directly, or alternatively you may use the LowerHex and UpperHex impls to
/// format it as a hex color string.
///
/// - `format!("#{:x}", color)`&ensp;&#10230;&ensp;`#f0f0f0`
/// - `format!("#{:X}", color)`&ensp;&#10230;&ensp;`#F0F0F0`
#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// `#000000`
impl Default for Color {
    fn default() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }
}

pub(crate) const fn color(n: u32) -> Color {
    Color {
        r: ((n >> 16) & 0xff) as u8,
        g: ((n >> 8) & 0xff) as u8,
        b: (n & 0xff) as u8,
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color(#{:x})", self)
    }
}

impl LowerHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl UpperHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}
