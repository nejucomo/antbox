//! A trait and newtype enabling safe temporary ownership behind an `&mut` reference
#![deny(unsafe_code, missing_docs)]

mod mip;
mod optimpl;
mod slot;

pub use self::mip::MapInPlace;
pub use self::slot::MoveSlot;
