use antbox_color::Color;
use antbox_float::NNF;
use antbox_geom::{Angle, Point, Shape};

use crate::{Renderable, TransformationLayer};

/// A simple abstract immediate-rendering backend
pub trait Backend {
    /// Render any [Renderable] value
    fn render<R: Renderable>(&mut self, r: R) {
        r.render_to(self);
    }

    /// Wrap `&mut self` into a [TransformationLayer] which transforms all shapes
    fn transformation_layer<A, S, P>(
        &mut self,
        angle: A,
        scale: S,
        translation: P,
    ) -> TransformationLayer<'_, Self>
    where
        A: Into<Angle>,
        S: Into<NNF>,
        P: Into<Point>,
    {
        TransformationLayer::new(self, angle.into(), scale.into(), translation.into())
    }

    /// Clear the screen with the given `color`
    fn clear_screen(&mut self, color: Color);

    /// Render the given [Shape] with [Color]
    fn render_shape_and_color(&mut self, shape: Shape, color: Color);
}
