#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs)]

mod optext;
mod slot;
mod starg;
mod term_starg;

pub mod into;
pub mod mutable;
pub mod take_into;
pub use self::slot::Slot;
pub use self::starg::Starg;
pub use self::term_starg::TermStarg;
