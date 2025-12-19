//! Provides [State] and supporting types for functional, I/O-free `antbox` state evolution
#![deny(unsafe_code, missing_docs)]

mod ant;
mod anthole;
pub(crate) mod consts;
mod food;
mod genparams;
mod object;
mod objectish;
mod optinto;
mod pheromone;
mod randutil;
mod spot;
mod spotupdate;
mod state;
mod steppedupon;
pub(crate) mod wcoin;

pub use self::ant::Ant;
pub use self::anthole::AntHole;
pub use self::food::Food;
pub use self::genparams::GenParams;
pub use self::object::Object;
pub use self::objectish::Objectish;
pub use self::optinto::OptInto;
pub use self::pheromone::{Pheromone, Pheromones};
pub use self::spot::Spot;
pub use self::state::State;
pub use self::steppedupon::SteppedUpon;
