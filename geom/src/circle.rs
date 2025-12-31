use antbox_float::NNF;
use derive_new::new;

use crate::{Angle, Distance, Point, Transformable};

/// A renderable [Circle]
#[derive(Copy, Clone, Debug, new)]
pub struct Circle {
    /// The `center` [Point] of the [Circle]
    #[new(into)]
    pub center: Point,
    /// The `radius` of the [Circle]
    #[new(into)]
    pub radius: Distance,
}

impl Transformable for Circle {
    fn rotate_by_angle(self, a: Angle) -> Self {
        Circle {
            center: self.center.rotate_by_angle(a),
            ..self
        }
    }

    fn scale_by_nnf(self, s: NNF) -> Self {
        Circle {
            radius: self.radius * s,
            ..self
        }
    }

    fn translate_by_point(self, p: Point) -> Self {
        Circle {
            center: self.center.translate_by_point(p),
            ..self
        }
    }
}
