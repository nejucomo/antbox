//! An API simplification layer over [speedy2d::window] layer
//!
//! In particular, this reifies the [speedy2d::window::WindowHandler::on_start] semantics into the type system: The application passes [WindowHandlerParams] to [WindowExt::run_loop_simplified] and on the start event those params construct the [WindowHandlerSimplified] which removes internal "started/not-started" state tracking from the application. Additionally some event handling uses a more ergonomic API.
#![deny(unsafe_code, missing_docs)]

mod adapter;
mod handler;
mod winext;

pub mod event;
pub use self::handler::{WindowHandlerParams, WindowHandlerSimplified};
pub use self::winext::WindowExt;
