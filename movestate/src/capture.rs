//! Input capturing adapters to convert `TakeIntoNext<I> -> IntoNext`

mod copy;

pub use self::copy::CaptureCopy;
