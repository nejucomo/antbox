//! Extends `antbox` game state along with animation states
//!
//! # TODO
//!
//! - Move the stateful `FoodDecoration` into `antbox-state` so that ants interact with it first-class.
#![deny(unsafe_code, missing_docs)]

mod anim;
mod colors;
mod drawable;
mod drawantbox;
mod gfxlayout;
mod gridlayout;
pub mod layers;
mod rectext;

pub use self::anim::AnimationState;
pub use self::drawable::Drawable;
pub use self::gfxlayout::GfxLayout;
pub use self::gridlayout::GridLayout;
pub use self::rectext::RectExt;
