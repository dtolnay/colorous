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
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
}

/// `#000000`
impl Default for Color {
    fn default() -> Self {
        Color { r: 0, g: 0, b: 0 }
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
