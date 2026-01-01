use antbox_float::NNF;
use derive_new::new;

use crate::{Angle, Distance, Point, Transformable, Vector};

/// A [Line] is a [Vector] with [width](Self::width)
#[derive(Copy, Clone, Debug, new)]
pub struct Line {
    /// The [Vector]
    #[new(into)]
    pub vec: Vector,
    /// The width
    #[new(into)]
    pub width: Distance,
}

impl Transformable for Line {
    fn rotate_by_angle(self, a: Angle) -> Self {
        Line {
            vec: self.vec.rotate_by_angle(a),
            ..self
        }
    }

    fn scale_by_nnf(self, s: NNF) -> Self {
        Line {
            vec: self.vec.scale_by_nnf(s),
            width: self.width * s,
        }
    }

    fn translate_by_point(self, p: Point) -> Self {
        Line {
            vec: self.vec.translate_by_point(p),
            ..self
        }
    }
}
