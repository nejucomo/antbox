//! A library for Mealy-machine functional state transitions and encapsulation within a mutable [Slot]
//!
//! # TODO
//!
//! Write a motivation section showing some simple FSM patterns on `&mut self` and how they can be cumbersome.
#![deny(unsafe_code, missing_docs)]

mod intonext;
mod optext;
mod slot;
mod upin;
mod upio;

pub use self::intonext::IntoNext;
pub use self::slot::Slot;
pub use self::upin::UpdateInput;
pub use self::upio::UpdateIO;
