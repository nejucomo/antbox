use antbox_s2render::{RenderCycle, Renderable};

use crate::colors;

/// The background layer
#[derive(Debug)]
pub struct Background;

impl Renderable for Background {
    fn schedule(self, cycle: &mut RenderCycle) {
        colors::DIRT.schedule(cycle);
    }
}
