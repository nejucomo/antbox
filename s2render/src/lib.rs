//! An [antbox_render::Backend] impl for [speedy2d]
#![deny(unsafe_code, missing_docs)]

mod into_s2;
mod s2backend;

pub use self::s2backend::Speedy2Backend;
