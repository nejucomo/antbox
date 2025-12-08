//! A library for functional state transitions and a mutable [Slot]
#![deny(unsafe_code, missing_docs)]

mod iotrans;
mod optext;
mod slot;

pub use self::iotrans::IOTransform;
pub use self::slot::Slot;
