use antbox_color::Color;
use antbox_float::NNF;
use antbox_geom::{Angle, Point, Shape, Transformable as _};
use derive_new::new;

use crate::Backend;

/// Transform all [Shape]s when rendering to an inner [Backend]
#[derive(Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct TransformationLayer<'a, B: ?Sized + Backend> {
    inner: &'a mut B,
    angle: Angle,
    scale: NNF,
    translation: Point,
}

impl<'a, B: ?Sized + Backend> Backend for TransformationLayer<'a, B> {
    fn clear_screen(&mut self, color: Color) {
        self.inner.clear_screen(color);
    }

    fn render_shape_and_color(&mut self, shape: Shape, color: Color) {
        self.inner.render_shape_and_color(
            shape
                .rotate_by_angle(self.angle)
                .scale_by_nnf(self.scale)
                .translate_by_point(self.translation),
            color,
        );
    }
}
