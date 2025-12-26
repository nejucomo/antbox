#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs)]

mod into_halting_state;
mod into_halting_stout;
mod into_map_state;
mod into_next;
mod into_state;
mod into_stout;
mod stditer;
mod take_into_halting_state;
mod take_into_halting_stout;
mod take_into_map_state;
mod take_into_next;
mod take_into_state;
mod take_into_stout;

pub mod combinators;
pub mod next;
pub use self::into_halting_state::IntoHaltingState;
pub use self::into_halting_stout::IntoHaltingStout;
pub use self::into_map_state::IntoMapState;
pub use self::into_next::IntoNext;
pub use self::into_state::IntoState;
pub use self::into_stout::IntoStout;
pub use self::take_into_halting_state::TakeIntoHaltingState;
pub use self::take_into_halting_stout::TakeIntoHaltingStout;
pub use self::take_into_map_state::TakeIntoMapState;
pub use self::take_into_next::TakeIntoNext;
pub use self::take_into_state::TakeIntoState;
pub use self::take_into_stout::TakeIntoStout;
pub mod mutable;

#[cfg(test)]
mod tests;
