use std::cmp::Ordering;

use derive_new::new;

use crate::{Layer, RenderQueue, Renderable, ShapeWithColor};

/// The unit of rendering
#[derive(Copy, Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct Element {
    layer: Layer,
    shwico: ShapeWithColor,
}

impl Element {
    pub(crate) fn shwico(self) -> ShapeWithColor {
        self.shwico
    }
}

impl Renderable for Element {
    fn render_to(self, rq: &mut RenderQueue) {
        rq.schedule_element(self);
    }
}

impl Eq for Element {}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.layer == other.layer
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        self.layer.cmp(&other.layer)
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
