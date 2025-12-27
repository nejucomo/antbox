//! Useful [Next](crate::TakeIntoNext::Next) types

mod halting;
mod map_state;
mod state;
mod stout;

pub use self::halting::Halting;
pub use self::map_state::MapState;
pub use self::state::State;
pub use self::stout::Stout;
