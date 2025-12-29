use antbox_geom::Rect;
use derive_new::new;

use crate::{Backend, Color, LayerScheduler, RenderScheduler, Renderable};

/// A [RenderCycle] encapsulates [schedul](RenderCycle::schedule)ing any number of [Renderable]s, then [render](RenderCycle::render)ing them all.
#[derive(Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct RenderCycle<'a> {
    rs: &'a mut RenderScheduler,
    view_size: Rect,
    #[new(default)]
    done: bool,
}

impl<'a> RenderCycle<'a> {
    /// Get the target view size
    pub fn view_size(&self) -> Rect {
        self.view_size
    }

    /// Schedule the given [Renderable]
    pub fn schedule<R>(&mut self, r: R)
    where
        R: Renderable,
    {
        r.schedule(self);
    }

    /// Get the layer scheduler for the give layer
    pub fn get_layer(&mut self, layer: usize) -> &mut LayerScheduler {
        self.rs.get_layer(layer)
    }

    /// Render all scheduled elements, draining the queue
    pub fn render<B>(mut self, gfx: &mut B)
    where
        B: Backend,
    {
        self.rs.render(gfx);
        self.done = true;
    }

    /// Schedule the element to be drawn
    pub(crate) fn schedule_bg_color(&mut self, color: Color) {
        self.rs.schedule_bg_color(color);
    }
}

impl<'a> Drop for RenderCycle<'a> {
    fn drop(&mut self) {
        assert!(self.done);
    }
}
