use core::fmt::Display;

#[derive(Clone, Copy)]
pub struct Color(u32);

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        let mut color = 0u32;
        color |= (r as u32) << 24;
        color |= (g as u32) << 16;
        color |= (b as u32) << 8;
        color |= a as u32;
        Self(color)
    }

    /// Returns this `Color` as a `u32`, but in big-endian format
    ///
    /// ### Example
    /// A `Color` with hex value `#12345678` would be returned in the format:
    ///
    /// `0x [78] [56] [34] [12]`
    pub fn to_u32_be(self) -> u32 {
        self.0.to_be()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let a = self.0 & 0xFF;

        if a == 0xFF {
            write!(f, "#{:06x}", self.0 >> 8)
        } else {
            write!(f, "#{:08x}", self.0)
        }
    }
}
