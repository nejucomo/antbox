#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs)]

pub mod halting;
mod into_next;
pub mod state;
pub mod stout;
mod take_into_next;

pub use self::into_next::IntoNext;
pub use self::take_into_next::TakeIntoNext;
pub mod mutable;
