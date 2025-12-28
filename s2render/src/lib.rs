//! [Renderable] trait for layer-orderd (z-axis) rendering
#![deny(unsafe_code, missing_docs)]

mod drawonto;
mod rectext;
mod rendarg;
mod renderable;
mod scheduler;
mod shwico;
mod vec2ext;

pub use self::rectext::RectExt;
pub use self::rendarg::RenderWithArg;
pub use self::renderable::Renderable;
pub use self::scheduler::{LayerScheduler, RenderCycle, RenderScheduler};
pub use self::shwico::{ShapeWithColor, WithColor};
pub use self::vec2ext::Vec2Ext;
