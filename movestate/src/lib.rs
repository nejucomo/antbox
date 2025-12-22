#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs)]

mod optext;
mod slot;

pub mod into;
pub mod next;
pub mod toolkit;
pub mod update;
pub use self::slot::Slot;
