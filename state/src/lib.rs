//! Provides [State] and supporting types for functional, I/O-free `antbox` state evolution
#![deny(unsafe_code, missing_docs)]

mod genparams;
mod state;

pub use self::genparams::GenParams;
pub use self::state::State;
