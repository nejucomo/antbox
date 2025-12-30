//! Newtype wrappers for [f32]
#![deny(unsafe_code, missing_docs)]

mod error;
mod norm;

pub use self::error::BoundedFloatError;
pub use self::norm::NormF32;
