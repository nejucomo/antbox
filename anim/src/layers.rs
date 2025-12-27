//! Layers of the animation state and/or rendering
mod background;
mod wireframe;

use antbox_s2render::{LayerScheduler, RenderScheduler};

pub use self::background::Background;
pub use self::wireframe::WireFrame;

/// A [Layer] index for the z-axis
#[derive(Copy, Clone, Debug)]
pub enum Layer {
    #[allow(missing_docs)]
    AntHole = 0,
    #[allow(missing_docs)]
    Plants,
    #[allow(missing_docs)]
    Pheromones,
    #[allow(missing_docs)]
    Ants,
    #[allow(missing_docs)]
    WireFrame,
}

impl Layer {
    /// The total number of layers
    pub fn count() -> usize {
        Layer::WireFrame.as_usize() + 1
    }

    /// The [LayerScheduler] for this [Layer]
    pub fn layer_scheduler(self, rs: &mut RenderScheduler) -> &mut LayerScheduler {
        &mut rs[self.as_usize()]
    }

    fn as_usize(self) -> usize {
        self as usize
    }
}
