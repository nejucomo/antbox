//! `(S, I) -> *`: Traits for state transitions with input
mod next;
mod opt_starg;
mod starg;
mod term_starg;
mod update;

pub use self::next::TakeIntoNext;
pub use self::opt_starg::TakeIntoOptStarg;
pub use self::starg::TakeIntoStarg;
pub use self::term_starg::TakeIntoTermStarg;
pub use self::update::TakeIntoUpdate;
