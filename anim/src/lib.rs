//! Extends `antbox` game state along with animation states
#![deny(unsafe_code, missing_docs)]

mod anim;
mod colors;
mod drawable;
mod gfxlayout;
mod gridlayout;
pub mod layers;

pub use self::anim::AnimationState;
pub use self::drawable::Drawable;
pub use self::gfxlayout::GfxLayout;
pub use self::gridlayout::GridLayout;

const TICKS_PER_CONWAY: usize = 50;
