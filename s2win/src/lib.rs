//! An API simplification layer over [speedy2d::window] layer
//!
//! In particular, this reifies the [speedy2d::window::WindowHandler::on_start] semantics into the type system: The application passes [WindowEventHandler::Params] to [WindowExt::run_loop_simplified] and on the start event those params construct the [WindowEventHandler] which removes internal "started/not-started" state tracking from the application. Additionally some event handling uses a more ergonomic API.
#![deny(unsafe_code, missing_docs)]

mod adapter;
mod handler;
mod inner;
mod into_ab;
mod winext;

pub mod event;
pub use self::handler::WindowEventHandler;
pub use self::winext::WindowExt;
