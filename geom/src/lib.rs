//! Two-dimensional [f32] geometry for `antbox`
#![deny(unsafe_code, missing_docs)]

mod angle;
mod cartesian;
mod distance;
mod point;
mod polar;

pub use self::angle::Angle;
pub use self::cartesian::Cartesian;
pub use self::distance::Distance;
pub use self::point::{Point, PointPeer};
pub use self::polar::Polar;
