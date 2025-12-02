//! An abstract cellular automata engine
#![deny(unsafe_code, missing_docs)]

mod cell;
mod conwayslife;
mod evolve;

use antbox_geom::Grid;

pub use self::cell::Cell;
pub use self::conwayslife::ConwaysLife;
pub use self::evolve::Evolvable;

/// A [Generation] represents the state at "a point in time" in the cellular automaton
pub type Generation = Grid<Cell>;
