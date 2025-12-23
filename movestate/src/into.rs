//! `S -> *`: Traits for state transitions without input
mod next;
mod starg;
mod term_starg;
mod update;

pub use self::next::IntoNext;
pub use self::starg::IntoStarg;
pub use self::term_starg::IntoTermStarg;
pub use self::update::IntoUpdate;
