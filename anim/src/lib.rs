//! Extends `antbox` game state along with animation states
//!
//! # TODO
//!
//! - Move the stateful `FoodDecoration` into `antbox-state` so that ants interact with it first-class.
#![deny(unsafe_code, missing_docs)]
#![allow(dead_code)]

mod anim;
mod circle;
mod colors;
mod drawable;
mod gridlayout;
pub mod layers;
mod organic;
mod rectext;
mod vec2ext;
mod wyrgrid;

pub use self::anim::AnimationState;
pub use self::drawable::Drawable;
pub use self::gridlayout::GridLayout;
pub use self::rectext::RectExt;
pub use self::vec2ext::Vec2Ext;
pub use self::wyrgrid::WyrGrid;
