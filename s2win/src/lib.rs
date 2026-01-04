//! A 2d-graphics focused windowing SDK
//!
//! Applications implement [S2App].
#![deny(unsafe_code, missing_docs)]

mod adapter;
mod control;
mod inner;
mod into_ab;
mod s2app;

pub mod event;
pub use self::control::Control;
pub use self::s2app::S2App;
pub use speedy2d::window::UserEventSender;
