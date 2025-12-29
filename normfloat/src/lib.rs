//! [NF] newtype wrapping [f32] on the interval `[0, 1]`
#![deny(unsafe_code, missing_docs)]

mod error;
mod nf;

pub use self::error::NFError;
pub use self::nf::NF;
