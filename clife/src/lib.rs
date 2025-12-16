//! An abstract cellular automata engine
#![deny(unsafe_code, missing_docs)]

mod cell;
mod grid;
mod rule;

pub use self::cell::ConwayCell;
pub use self::grid::ConwayGrid;
pub use self::rule::conways_rule;
