#![deny(unsafe_code)]

mod adapter;
mod handler;
mod winext;

pub mod event;
pub use self::handler::{WindowHandlerParams, WindowHandlerSimplified};
pub use self::winext::WindowExt;
