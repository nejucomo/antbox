//! [Renderable] trait for layer-orderd (z-axis) rendering
#![deny(unsafe_code, missing_docs)]

mod backend;
mod rarg;
mod renderable;
mod renref;
mod sac;

pub use self::backend::Backend;
pub use self::rarg::RenderWithArg;
pub use self::renderable::Renderable;
pub use self::renref::RenderRefWithArg;
pub use self::sac::{Colorable, ShapeAndColor};
