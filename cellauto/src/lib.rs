//! An abstract cellular automata engine
#![deny(unsafe_code, missing_docs)]

mod cell;
mod conwayslife;
mod randgen;
mod ruleset;

use antbox_geom::Grid;

pub use self::cell::Cell;
pub use self::conwayslife::ConwaysLife;
pub use self::randgen::GenGen;
pub use self::ruleset::Ruleset;

/// A [Generation] represents the state at "a point in time" in the cellular automaton
pub type Generation = Grid<Cell>;
