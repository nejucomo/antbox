use crate::{Angle, Distance, Point, Transformable, Vector};

/// A [Rect]angle
///
/// # Diagonals
///
/// [Rect] is constructed from (and represented by) a [Vector] representing any diagonal of the rectangle. Internally it always stores top-left to bottom-right.
#[derive(Copy, Clone, Debug)]
pub struct Rect(Vector);

impl Rect {
    /// The top-left to borrom-right diagonal
    pub fn diagonal(self) -> Vector {
        self.0
    }

    /// The width and height
    pub fn width_and_height(self) -> (Distance, Distance) {
        let Point { x: w, y: h } = self.0.delta;
        (w.into(), h.into())
    }

    /// The center [Point]
    pub fn center(self) -> Point {
        self.diagonal().scale(0.5).to()
    }

    /// The minimum side length
    pub fn minimum_side_length(self) -> Distance {
        let (w, h) = self.width_and_height();
        w.min(h) * 0.5
    }

    /// The inner radius is half the minimum side length
    pub fn inner_radius(self) -> Distance {
        self.minimum_side_length() * 0.5
    }
}

impl Transformable for Rect {
    fn rotate<A>(self, a: A) -> Self
    where
        A: Into<Angle>,
    {
        Self(self.0.rotate(a))
    }

    fn scale(self, s: f32) -> Self {
        Self(self.0.scale(s))
    }

    fn translate(self, delta: Point) -> Self {
        Self(self.0.translate(delta))
    }
}

impl From<Vector> for Rect {
    fn from(diag: Vector) -> Self {
        Rect(if diag.delta.x >= 0.0 && diag.delta.y >= 0.0 {
            diag
        } else {
            let (x1, y1) = diag.start.into();
            let (x2, y2) = diag.to().into();
            let top_left = Point::new(x1.min(x2), y1.min(y2));
            let bottom_right = Point::new(x1.max(x2), y1.max(y2));
            top_left.vector_to(bottom_right)
        })
    }
}
