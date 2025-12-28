use std::collections::VecDeque;

use speedy2d::Graphics2D;
use speedy2d::color::Color;

use crate::{Renderable, ShapeWithColor};

/// A [RenderScheduler] sorts [ShapeWithColor]s by [LayerScheduler] to draw onto a [Graphics2D] in layer (z-axis) order
#[derive(Debug)]
pub struct RenderScheduler {
    bgslot: Option<Color>,
    layers: Vec<LayerScheduler>,
}

/// A [RenderCycle] encapsulates [schedul](RenderCycle::schedule)ing any number of [Renderable]s, then [render](RenderCycle::render)ing them all.
#[derive(Debug)]
pub struct RenderCycle<'a> {
    rs: &'a mut RenderScheduler,
    done: bool,
}

impl RenderScheduler {
    /// Construct a new [RenderScheduler] with `layers` distinct layers
    pub fn new(layers: usize) -> Self {
        RenderScheduler {
            bgslot: None,
            layers: {
                let mut v = Vec::with_capacity(layers);
                v.resize_with(layers, LayerScheduler::default);
                v
            },
        }
    }

    /// Start a new [RenderCycle]
    pub fn start_cycle(&mut self) -> RenderCycle<'_> {
        RenderCycle {
            rs: self,
            done: false,
        }
    }
}

impl<'a> RenderCycle<'a> {
    /// Schedule the given [Renderable]
    pub fn schedule<R>(&mut self, r: R)
    where
        R: Renderable,
    {
        r.schedule(self);
    }

    /// Get the layer scheduler for the give layer
    pub fn get_layer(&mut self, layer: usize) -> &mut LayerScheduler {
        &mut self.rs.layers[layer]
    }

    /// Render all scheduled elements, draining the queue
    pub fn render(mut self, gfx: &mut Graphics2D) {
        if let Some(bg) = self.rs.bgslot.take() {
            gfx.clear_screen(bg);
        }

        for layer in self.rs.layers.iter_mut() {
            while let Some(shwico) = layer.0.pop_front() {
                shwico.draw_onto(gfx);
            }
        }

        self.done = true;
    }

    /// Schedule the element to be drawn
    pub(crate) fn schedule_bg_color(&mut self, color: Color) {
        assert!(self.rs.bgslot.replace(color).is_none());
    }
}

impl<'a> Drop for RenderCycle<'a> {
    fn drop(&mut self) {
        assert!(self.done);
    }
}

/// Schedule [ShapeWithColor]s for a given layer
///
/// The render order within a [LayerScheduler] is guaranteed to occur in the order of [LayerScheduler::schedule] calls
#[derive(Debug, Default)]
pub struct LayerScheduler(VecDeque<ShapeWithColor>);

impl LayerScheduler {
    /// Schedule a [ShapeWithColor] on this layer
    pub fn schedule(&mut self, shwico: ShapeWithColor) {
        self.0.push_back(shwico);
    }
}
