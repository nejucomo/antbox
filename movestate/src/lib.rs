#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs)]

mod into_next;
mod take_into_next;

pub use self::into_next::IntoNext;
pub use self::take_into_next::TakeIntoNext;
pub mod mutable;
pub mod starg;
