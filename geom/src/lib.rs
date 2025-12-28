//! Two-dimensional geometry for `antbox`
#![deny(unsafe_code, missing_docs)]

mod angle;
mod distance;
mod polar;

pub use self::angle::Angle;
pub use self::distance::Distance;
pub use self::polar::Polar;
