use std::fmt::{self, Debug, LowerHex, UpperHex};

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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
        f.write_str("Color(#")?;
        LowerHex::fmt(self, f)?;
        f.write_str(")")
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
