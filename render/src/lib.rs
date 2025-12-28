//! [Renderable] trait for layer-orderd (z-axis) rendering
#![deny(unsafe_code, missing_docs)]

mod backend;
mod color;
mod rendarg;
mod renderable;
mod scheduler;
mod shwico;

pub use self::backend::Backend;
pub use self::color::Color;
pub use self::rendarg::RenderWithArg;
pub use self::renderable::Renderable;
pub use self::scheduler::{LayerScheduler, RenderCycle, RenderScheduler};
pub use self::shwico::{ShapeWithColor, WithColor};
