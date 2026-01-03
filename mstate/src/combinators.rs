//! Types composed from [TakeIntoNext](crate::TakeIntoNext) methods which also provide [TakeIntoNext](crate::TakeIntoNext) impls

mod capture_clone;
mod capture_copy;

pub use self::capture_clone::CaptureClone;
pub use self::capture_copy::CaptureCopy;
