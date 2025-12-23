//! `(S, I) -> *`: Traits for state transitions with input
mod next;
mod starg;
mod term_starg;
mod update;

pub use self::next::TakeIntoNext;
pub use self::starg::TakeIntoStarg;
pub use self::term_starg::TakeIntoTermStarg;
pub use self::update::TakeIntoUpdate;
