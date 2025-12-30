//! Color representation in `antbox`
#![deny(unsafe_code, missing_docs)]

use antbox_float::NormF32;

macro_rules! def_rgba {
    ( $t:ident, $band:ty, $maker_alpha:ident, $maker_noa:ident, $default_alpha:expr ) => {
        #[doc = concat!("An _RGBA_ color representation with [", stringify!($band), "] bands")]
        pub struct $t {
            /// The red band
            pub r: $band,
            /// The green band
            pub g: $band,
            /// The blue band
            pub b: $band,
            /// The alpha band
            pub a: $band,
        }

        impl $t {
            /// Construct with all four bands
            pub const fn new(r: $band, g: $band, b: $band, a: $band) -> Self {
                Self { r, g, b, a }
            }

            /// Construct with all _RGB_ bands and full alpha
            pub const fn new_full_alpha(r: $band, g: $band, b: $band) -> Self {
                Self::new(r, g, b, $default_alpha)
            }
        }

        #[doc = concat!("A shorthand for [", stringify!($t), "::new]")]
        pub const fn $maker_alpha(r: $band, g: $band, b: $band, a: $band) -> $t {
            $t::new(r, g, b, a)
        }

        #[doc = concat!("A shorthand for [", stringify!($t), "::new_full_alpha]")]
        pub const fn $maker_noa(r: $band, g: $band, b: $band) -> $t {
            $t::new_full_alpha(r, g, b)
        }
    };
}

def_rgba!(RGBAu, u8, rgba, rgb, u8::MAX);
def_rgba!(RGBAf, NormF32, rgba_f, rgb_f, NormF32::from_f32(1.0));
