//! Range-bounded [f32] newtypes for intervals `[0, 1]`, `[0, ∞]`, `(0, ∞)`
#![deny(unsafe_code, missing_docs)]

mod error;
mod norm;

pub use self::error::BoundedFloatError;
pub use self::norm::NormF32;
