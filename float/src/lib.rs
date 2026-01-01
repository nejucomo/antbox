//! Newtype wrappers for [f32]
#![deny(unsafe_code, missing_docs)]

mod error;
mod nnf;
mod norm;
mod powu;

pub use self::error::BoundedFloatError;
pub use self::nnf::NNF;
pub use self::norm::Norm;
pub use self::powu::PowUnsigned;
