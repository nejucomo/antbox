//! A simple 2D integer geometry crate for `antbox`
#![deny(unsafe_code, missing_docs)]

mod boundpoint;
mod bounds;
mod direction;
mod grid;
mod point;

pub use crate::boundpoint::BoundPoint;
pub use crate::bounds::Bounds;
pub use crate::direction::Direction;
pub use crate::grid::Grid;
pub use crate::point::Point;
