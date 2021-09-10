use core::fmt::{self, Debug, LowerHex, UpperHex};

/// &#8203;
///
/// `Color` has public fields `r`, `g`, `b` of type `u8` which you can read
/// directly, or alternatively you may use the LowerHex and UpperHex impls to
/// format it as a hex color string.
///
/// - `format!("#{:x}", color)`&ensp;&#10230;&ensp;`#f0f0f0`
/// - `format!("#{:X}", color)`&ensp;&#10230;&ensp;`#F0F0F0`
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
}

impl Color {
    pub fn as_tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    pub fn into_tuple(self) -> (u8, u8, u8) {
        self.as_tuple()
    }

    pub fn as_array(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }

    pub fn into_array(self) -> [u8; 3] {
        self.as_array()
    }
}

/// `#000000`
#[allow(clippy::derivable_impls)] // https://github.com/rust-lang/rust-clippy/issues/7654
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
