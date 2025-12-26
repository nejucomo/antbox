//! Input capturing adapters to convert `TakeIntoNext<I> -> IntoNext`

mod clone;
mod copy;

pub use self::clone::CaptureClone;
pub use self::copy::CaptureCopy;
