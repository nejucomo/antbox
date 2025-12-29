use std::collections::VecDeque;

use antbox_geom::Rect;

use crate::{Backend, Color, RenderCycle, ShapeWithColor};

/// A [RenderScheduler] sorts [ShapeWithColor]s by [LayerScheduler] to draw onto a [Backend] in layer (z-axis) order
#[derive(Debug)]
pub struct RenderScheduler {
    bgslot: Option<Color>,
    layers: Vec<LayerScheduler>,
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
    pub fn start_cycle(&mut self, view_size: Rect) -> RenderCycle<'_> {
        RenderCycle::new(self, view_size)
    }

    // [RenderCycle] interface:
    pub(crate) fn schedule_bg_color(&mut self, color: Color) {
        assert!(self.bgslot.replace(color).is_none());
    }

    pub(crate) fn get_layer(&mut self, layer: usize) -> &mut LayerScheduler {
        &mut self.layers[layer]
    }

    pub(crate) fn render<B>(&mut self, gfx: &mut B)
    where
        B: Backend,
    {
        if let Some(bg) = self.bgslot.take() {
            gfx.clear_screen(bg);
        }

        for layer in self.layers.iter_mut() {
            while let Some(shwico) = layer.0.pop_front() {
                shwico.render_to(gfx);
            }
        }
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
