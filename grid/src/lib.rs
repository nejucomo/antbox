//! A simple 2D integer geometry crate for `antbox`
#![deny(unsafe_code, missing_docs)]

mod bounds;
mod coord;
mod direction;
mod dirset;
mod gcoord;
mod grid;

pub use crate::bounds::Bounds;
pub use crate::coord::Coord;
pub use crate::direction::{DirIter, Direction};
pub use crate::dirset::{DirSet, DirSetIter};
pub use crate::gcoord::GridCoord;
pub use crate::grid::Grid;

#[cfg(test)]
mod tests;
