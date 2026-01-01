use antbox_color::Color;
use antbox_float::NNF;
use antbox_geom::{Angle, Point, Shape, Transformable};
use derive_debug::Dbg;
use derive_new::new;

use crate::Backend;

/// Transform all [Shape]s when rendering to an inner [Backend]
#[derive(Dbg, new)]
#[new(visibility = "pub(crate)")]
pub struct TransformationLayer<'a, B: ?Sized + Backend> {
    #[dbg(placeholder = "...")]
    inner: &'a mut B,
    #[new(value = "Angle::from(0.0)")]
    angle: Angle,
    #[new(value = "NNF::ONE")]
    scale: NNF,
    #[new(value = "Point::ORIGIN")]
    translation: Point,
}

impl<'a, B: ?Sized + Backend> Transformable for TransformationLayer<'a, B> {
    fn rotate_by_angle(self, a: Angle) -> Self {
        Self {
            angle: self.angle + a,
            ..self
        }
    }

    fn scale_by_nnf(self, s: NNF) -> Self {
        Self {
            scale: self.scale * s,
            ..self
        }
    }

    fn translate_by_point(self, p: Point) -> Self {
        Self {
            translation: self.translation + p,
            ..self
        }
    }
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
