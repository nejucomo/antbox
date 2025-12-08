//! A library for functional state transitions and a mutable [Slot]
#![deny(unsafe_code, missing_docs)]

mod optext;
mod slot;
mod updateio;

pub use self::slot::Slot;
pub use self::updateio::{UpdateIO, UpdateInput};
