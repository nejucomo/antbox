//! A library for Mealy-machine functional state transitions and encapsulation within a mutable [Slot]
//!
//! # TODO
//!
//! Write a motivation section showing some simple FSM patterns on `&mut self` and how they can be cumbersome.
#![deny(unsafe_code, missing_docs)]

mod optext;
mod slot;
mod updateio;

pub use self::slot::Slot;
pub use self::updateio::{UpdateIO, UpdateInput};
