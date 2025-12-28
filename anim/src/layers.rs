//! Layers of the animation state and/or rendering
mod background;
mod wireframe;

use antbox_render::{LayerScheduler, RenderCycle};

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
        1 + Layer::WireFrame as usize
    }

    /// The [LayerScheduler] for this [Layer]
    pub fn scheduler<'a>(self, cycle: &'a mut RenderCycle) -> &'a mut LayerScheduler {
        cycle.get_layer(self as usize)
    }
}
