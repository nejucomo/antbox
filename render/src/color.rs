use crate::{RenderCycle, Renderable};

/// An `RGBA` color representation with bands in [0, 1]
#[derive(Copy, Clone, Debug)]
pub struct Color {
    #[allow(missing_docs)]
    pub r: f32,
    #[allow(missing_docs)]
    pub g: f32,
    #[allow(missing_docs)]
    pub b: f32,
    #[allow(missing_docs)]
    pub a: f32,
}

impl Renderable for Color {
    fn schedule(self, cycle: &mut RenderCycle) {
        cycle.schedule_bg_color(self);
    }
}
