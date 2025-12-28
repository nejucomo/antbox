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
    fn rotate<A>(self, a: A) -> Self
    where
        A: Into<Angle>,
    {
        use Shape::*;

        match self {
            Circle(x) => Circle(x.rotate(a)),
            Line(x) => Line(x.rotate(a)),
            Rect(x) => Rect(x.rotate(a)),
        }
    }

    fn scale(self, s: f32) -> Self {
        use Shape::*;

        match self {
            Circle(x) => Circle(x.scale(s)),
            Line(x) => Line(x.scale(s)),
            Rect(x) => Rect(x.scale(s)),
        }
    }

    fn translate(self, delta: Point) -> Self {
        use Shape::*;

        match self {
            Circle(x) => Circle(x.translate(delta)),
            Line(x) => Line(x.translate(delta)),
            Rect(x) => Rect(x.translate(delta)),
        }
    }
}
