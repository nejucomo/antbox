//! `S -> *`: Traits for state transitions without input
mod next;
mod opt_starg;
mod starg;
mod term_starg;
mod update;

pub use self::next::IntoNext;
pub use self::opt_starg::IntoOptStarg;
pub use self::starg::IntoStarg;
pub use self::term_starg::IntoTermStarg;
pub use self::update::IntoUpdate;
