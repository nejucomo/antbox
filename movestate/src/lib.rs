//! Building blocks for state evolution using move semantics; e.x. `(S, I) -> (S, O)`
//!
//! # TODO
//!
//! Write a motivation section with examples
#![deny(unsafe_code, missing_docs)]

mod optext;
mod slot;

pub mod into;
pub mod toolkit;
pub use self::slot::Slot;
