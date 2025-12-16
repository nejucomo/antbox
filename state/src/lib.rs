//! Provides [State] and supporting types for functional, I/O-free `antbox` state evolution
#![deny(unsafe_code, missing_docs)]

mod genparams;
mod object;
mod spot;
mod state;

pub use self::genparams::GenParams;
pub use self::object::Object;
pub use self::spot::Spot;
pub use self::state::State;
