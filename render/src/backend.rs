use antbox_color::Color;
use antbox_geom::Shape;

use crate::{Renderable, TransformationLayer};

/// A simple abstract immediate-rendering backend
pub trait Backend {
    /// Render any [Renderable] value
    fn render<R: Renderable>(&mut self, r: R) {
        r.render_to(self);
    }

    /// Wrap `&mut self` into a [TransformationLayer] which transforms all shapes
    fn transformation_layer(&mut self) -> TransformationLayer<'_, Self> {
        TransformationLayer::new(self)
    }

    /// Clear the screen with the given `color`
    fn clear_screen(&mut self, color: Color);

    /// Render the given [Shape] with [Color]
    fn render_shape_and_color(&mut self, shape: Shape, color: Color);
}
