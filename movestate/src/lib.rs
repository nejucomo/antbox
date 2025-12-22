//! Building blocks for state evolution using move semantics; e.x. `(S, I) -> (S, O)`
//!
//! # TODO
//!
//! Write a motivation section with examples
#![deny(unsafe_code, missing_docs)]

mod optext;
mod slot;

pub mod into;
pub mod next;
pub mod toolkit;
pub mod update;
pub use self::slot::Slot;
