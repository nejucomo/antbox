#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs)]

mod halting;
mod into_next;
mod map_state;
mod state;
mod stditer;
mod stout;
mod take_into_next;

pub use self::halting::{
    Halting, IntoHaltingState, IntoHaltingStout, TakeIntoHaltingState, TakeIntoHaltingStout,
};
pub use self::into_next::IntoNext;
pub use self::state::{IntoState, State, TakeIntoState};
pub use self::stout::{IntoStout, Stout, TakeIntoStout};
pub use self::take_into_next::TakeIntoNext;
pub mod mutable;
pub use self::map_state::MapState;

#[cfg(test)]
mod tests;
