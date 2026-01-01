use antbox_float::{NNF, Norm};

use crate::{Angle, Dimensions, Distance, Point, Transformable, Vector};

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

    /// The [Dimensions]
    pub fn dimensions(self) -> Dimensions {
        Dimensions::fromp_point(self.0.delta)
    }

    /// The center [Point]
    pub fn center(self) -> Point {
        self.diagonal().scale(Norm::HALF).to()
    }

    /// The minimum side length
    pub fn minimum_side_length(self) -> Distance {
        let Dimensions { width, height } = self.dimensions();
        width.min(height) * Norm::HALF
    }

    /// The inner radius is half the minimum side length
    pub fn inner_radius(self) -> Distance {
        self.minimum_side_length() * Norm::HALF
    }
}

impl Transformable for Rect {
    fn rotate_by_angle(self, a: Angle) -> Self {
        Self(self.0.rotate_by_angle(a))
    }

    fn scale_by_nnf(self, s: NNF) -> Self {
        Self(self.0.scale_by_nnf(s))
    }

    fn translate_by_point(self, p: Point) -> Self {
        Self(self.0.translate_by_point(p))
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
