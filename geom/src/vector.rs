use antbox_float::NNF;
use derive_new::new;

use crate::{Angle, Distance, Line, Point, Transformable};

/// A [Vector] [start](Self::start)s at a [Point] and proceeds to another point by a [delta](Self::delta)
#[derive(Copy, Clone, Debug, new)]
pub struct Vector {
    /// The starting [Point]
    #[new(into)]
    pub start: Point,
    /// The delta [Point]; the line ends at [`start`](Self::start) `+` [`delta`](Self::delta)
    #[new(into)]
    pub delta: Point,
}

impl Vector {
    /// The [Line] defined by `self` with `width`
    pub fn with_width<W>(self, width: W) -> Line
    where
        W: Into<Distance>,
    {
        Line::new(self, width)
    }

    /// The absolute end [Point] of `self`, ie `self.start + self.delta`
    pub fn to(self) -> Point {
        self.start + self.delta
    }
}

impl Transformable for Vector {
    fn rotate_by_angle(self, a: Angle) -> Self {
        Vector {
            delta: self.delta.rotate_by_angle(a),
            ..self
        }
    }

    fn scale_by_nnf(self, s: NNF) -> Self {
        Vector {
            delta: self.delta.scale_by_nnf(s),
            ..self
        }
    }

    fn translate_by_point(self, p: Point) -> Self {
        Vector {
            start: self.start + p,
            ..self
        }
    }
}
