#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs)]

mod intonext;
mod optext;
mod slot;
mod starg;
mod takeintonext;
mod takeintoupdate;
mod termstarg;

pub use self::intonext::IntoNext;
pub use self::slot::Slot;
pub use self::starg::Starg;
pub use self::takeintonext::TakeIntoNext;
pub use self::takeintoupdate::TakeIntoUpdate;
pub use self::termstarg::TermStarg;
