//! Provides [State] and supporting types for functional, I/O-free `antbox` state evolution
#![deny(unsafe_code, missing_docs)]

mod ant;
mod genparams;
mod object;
mod pheromone;
mod randutil;
mod spot;
mod state;

pub use self::ant::Ant;
pub use self::genparams::GenParams;
pub use self::object::{Object, Objectish};
pub use self::pheromone::{Pheromone, Pheromones};
pub use self::spot::Spot;
pub use self::state::State;
