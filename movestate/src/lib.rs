//! Building blocks for state evolution using move semantics; e.x. `(S, I) -> (S, O)`
//!
//! # TODO
//!
//! Write a motivation section with examples
#![deny(unsafe_code, missing_docs)]

mod intonext;
mod optext;
mod optupdate;
mod slot;
pub mod toolkit;
mod transform;
mod update;

pub use self::intonext::IntoNext;
pub use self::optupdate::OptUpdate;
pub use self::slot::Slot;
pub use self::transform::Transform;
pub use self::update::Update;
