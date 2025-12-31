//! Color representation in `antbox`
#![deny(unsafe_code, missing_docs)]

use antbox_float::Norm;

#[allow(missing_docs)]
pub const BLACK: Color = rgb(0, 0, 0);

/// An _RGBA_ color representation with [Norm] bands
pub struct Color {
    /// The red band
    pub r: Norm,
    /// The green band
    pub g: Norm,
    /// The blue band
    pub b: Norm,
    /// The alpha band
    pub a: Norm,
}

impl Color {
    /// Construct with all four bands
    pub const fn new(r: Norm, g: Norm, b: Norm, a: Norm) -> Self {
        Self { r, g, b, a }
    }

    /// Construct with all _RGB_ bands and full alpha
    pub const fn new_full_alpha(r: Norm, g: Norm, b: Norm) -> Self {
        Self::new(r, g, b, Norm::fromp_f32(1.0))
    }

    /// Construct from [u8] bands
    pub const fn new_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(
            Norm::from_u8(r),
            Norm::from_u8(g),
            Norm::from_u8(b),
            Norm::from_u8(a),
        )
    }

    /// Construct from  _RGB_ [u8] bands
    pub const fn new_u8_full_alpha(r: u8, g: u8, b: u8) -> Self {
        Self::new_u8(r, g, b, u8::MAX)
    }

    /// Interpolate linearly between two colors
    pub const fn interpolate(self, other: Color, proportion: Norm) -> Color {
        Color {
            r: self.r.interpolate(other.r, proportion),
            g: self.g.interpolate(other.g, proportion),
            b: self.b.interpolate(other.b, proportion),
            a: self.a.interpolate(other.a, proportion),
        }
    }
}

/// Shorthand for [Color::new_u8]
pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::new_u8(r, g, b, a)
}

/// Shorthand for [Color::new_u8_full_alpha]
pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new_u8_full_alpha(r, g, b)
}
