use derive_new::new;

use crate::{Angle, Distance, Point, Transformable, Vector};

/// A [Line] is a [Vector] with [width](Self::width)
#[derive(Copy, Clone, Debug, new)]
pub struct Line {
    /// The [Vector]
    #[new(into)]
    pub seg: Vector,
    /// The width
    #[new(into)]
    pub width: Distance,
}

impl Transformable for Line {
    fn rotate<A>(self, a: A) -> Self
    where
        A: Into<Angle>,
    {
        Line {
            seg: self.seg.rotate(a),
            ..self
        }
    }

    fn scale(self, s: f32) -> Self {
        Line {
            seg: self.seg.scale(s),
            width: self.width * s,
        }
    }

    fn translate(self, delta: Point) -> Self {
        Line {
            seg: self.seg.translate(delta),
            ..self
        }
    }
}
