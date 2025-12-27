//! [Renderable] trait for [Layer]-orderd (z-axis) rendering
#![deny(unsafe_code, missing_docs)]

mod circle;
mod drawonto;
mod element;
mod layer;
mod queue;
mod rectext;
mod renderable;
mod shape;
mod shwico;
mod vec2ext;

pub use self::circle::Circle;
pub use self::element::Element;
pub use self::layer::Layer;
pub use self::queue::RenderQueue;
pub use self::rectext::RectExt;
pub use self::renderable::Renderable;
pub use self::shape::Shape;
pub use self::shwico::{ShapeWithColor, WithColor};
pub use self::vec2ext::Vec2Ext;
