//! A simple 2D geometry crate for `antbox`
#![deny(unsafe_code, missing_docs)]

mod bounds;
mod point;
mod rect;

pub use crate::bounds::Bounds;
pub use crate::point::Point;
pub use crate::rect::Rect;
