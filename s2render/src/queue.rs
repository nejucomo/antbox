use std::collections::BinaryHeap;

use speedy2d::Graphics2D;

use crate::{Element, Renderable};

/// A [RenderQueue] sorts [Element]s by [Layer] to draw onto a [Graphics2D]
#[derive(Debug, Default)]
pub struct RenderQueue(BinaryHeap<Element>);

impl RenderQueue {
    /// Render all scheduled [Element]s, draining the queue
    pub fn render(&mut self, gfx: &mut Graphics2D) {
        while let Some(elem) = self.0.pop() {
            elem.shwico().draw_onto(gfx)
        }
    }

    /// Enqueue a [Renderable] object
    pub fn enqueue<R>(&mut self, r: R)
    where
        R: Renderable,
    {
        r.render_to(self);
    }

    /// Schedule the element to be drawn
    pub(crate) fn schedule_element(&mut self, element: Element) {
        self.0.push(element);
    }
}
