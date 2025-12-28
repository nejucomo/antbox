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
    fn rotate<A>(self, a: A) -> Self
    where
        A: Into<Angle>,
    {
        Circle {
            center: self.center.rotate(a),
            ..self
        }
    }

    fn scale(self, s: f32) -> Self {
        Circle {
            radius: self.radius * s,
            ..self
        }
    }

    fn translate(self, delta: Point) -> Self {
        Circle {
            center: self.center.translate(delta),
            ..self
        }
    }
}
