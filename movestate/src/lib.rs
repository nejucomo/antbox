#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs)]

mod into_next;
mod optext;
mod slot;
mod starg;
mod take_into_next;
mod take_into_update;
mod term_starg;

pub use self::into_next::IntoNext;
pub use self::slot::Slot;
pub use self::starg::Starg;
pub use self::take_into_next::TakeIntoNext;
pub use self::take_into_update::TakeIntoUpdate;
pub use self::term_starg::TermStarg;
