//! Extends `antbox` game state along with animation states
//!
//! # TODO
//!
//! - Move the stateful `FoodDecoration` into `antbox-state` so that ants interact with it first-class.
#![deny(unsafe_code, missing_docs)]
#![allow(dead_code)]

mod abrender;
mod anim;
mod colors;
mod gridlayout;
mod layers;
mod organic;
mod wyrgrid;

pub use self::anim::AnimationState;
