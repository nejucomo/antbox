//! A simple 2D geometry crate for `antbox`
#![deny(unsafe_code, missing_docs)]

mod boundpoint;
mod bounds;
mod grid;
mod point;
mod rect;
mod scalar;

pub use crate::boundpoint::BoundPoint;
pub use crate::bounds::Bounds;
pub use crate::grid::Grid;
pub use crate::point::Point;
pub use crate::rect::Rect;
pub use crate::scalar::Scalar;
