use antbox_float::NNF;
use derive_more::From;

use crate::{Angle, Circle, Line, Point, Rect, Transformable};

/// A [Shape]
#[derive(Copy, Clone, Debug, From)]
pub enum Shape {
    #[allow(missing_docs)]
    Circle(Circle),
    #[allow(missing_docs)]
    Line(Line),
    #[allow(missing_docs)]
    Rect(Rect),
}

impl Transformable for Shape {
    fn rotate_by_angle(self, a: Angle) -> Self {
        use Shape::*;

        match self {
            Circle(x) => Circle(x.rotate_by_angle(a)),
            Line(x) => Line(x.rotate_by_angle(a)),
            Rect(x) => Rect(x.rotate_by_angle(a)),
        }
    }

    fn scale_by_nnf(self, s: NNF) -> Self {
        use Shape::*;

        match self {
            Circle(x) => Circle(x.scale_by_nnf(s)),
            Line(x) => Line(x.scale_by_nnf(s)),
            Rect(x) => Rect(x.scale_by_nnf(s)),
        }
    }

    fn translate_by_point(self, p: Point) -> Self {
        use Shape::*;

        match self {
            Circle(x) => Circle(x.translate_by_point(p)),
            Line(x) => Line(x.translate_by_point(p)),
            Rect(x) => Rect(x.translate_by_point(p)),
        }
    }
}
