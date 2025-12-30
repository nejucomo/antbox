//! Color representation in `antbox`
#![deny(unsafe_code, missing_docs)]

use antbox_float::NormF32;

/// An _RBGA_ color representation with [NormF32] bands
pub struct RBGA {
    /// The red band
    pub r: NormF32,
    /// The green band
    pub g: NormF32,
    /// The blue band
    pub b: NormF32,
    /// The alpha band
    pub a: NormF32,
}

impl RBGA {
    /// Construct with all four bands
    pub const fn new(r: NormF32, g: NormF32, b: NormF32, a: NormF32) -> Self {
        Self { r, g, b, a }
    }

    /// Construct with all _RGB_ bands and full alpha
    pub const fn new_full_alpha(r: NormF32, g: NormF32, b: NormF32) -> Self {
        Self::new(r, g, b, NormF32::from_f32(1.0))
    }

    /// Construct from [u8] bands
    pub const fn new_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(
            NormF32::from_u8(r),
            NormF32::from_u8(g),
            NormF32::from_u8(b),
            NormF32::from_u8(a),
        )
    }

    /// Construct from  _RGB_ [u8] bands
    pub const fn new_u8_full_alpha(r: u8, g: u8, b: u8) -> Self {
        Self::new_u8(r, g, b, u8::MAX)
    }
}

/// Shorthand for [RBGA::new_u8]
pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> RBGA {
    RBGA::new_u8(r, g, b, a)
}

/// Shorthand for [RBGA::new_u8_full_alpha]
pub const fn rgb(r: u8, g: u8, b: u8) -> RBGA {
    RBGA::new_u8_full_alpha(r, g, b)
}
