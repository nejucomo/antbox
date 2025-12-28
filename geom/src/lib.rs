//! Two-dimensional [f32] geometry for `antbox`
//!
//! # Design
//!
//! - Public fields
//! - `::new` with `Into` parameters
//! - Construct composite items from constituent build methods, e.g. `Point::with_radius(...) -> Circle`
#![deny(unsafe_code, missing_docs)]

mod angle;
mod circle;
mod distance;
mod line;
mod point;
mod polar;
mod rect;
mod shape;
mod transformable;
mod vector;

pub use self::angle::Angle;
pub use self::circle::Circle;
pub use self::distance::Distance;
pub use self::line::Line;
pub use self::point::Point;
pub use self::polar::Polar;
pub use self::rect::Rect;
pub use self::shape::Shape;
pub use self::transformable::Transformable;
pub use self::vector::Vector;
