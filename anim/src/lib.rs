//! Extends `antbox` game state along with animation states
//!
//! # TODO
//!
//! - Move the stateful `FoodDecoration` into `antbox-state` so that ants interact with it first-class.
#![deny(unsafe_code, missing_docs)]
#![allow(dead_code)]

mod abrender;
mod antbox;
mod colors;
mod gridlayout;
mod layers;
mod organic;
mod runmode;
mod upev;
mod wyrgrid;

pub use self::antbox::AntboxAnimation;
pub use self::runmode::RunMode;
pub use self::upev::{UpdateEvent, UpdateSource};
