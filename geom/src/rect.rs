use antbox_float::{NNF, Norm};

use crate::{Angle, Circle, Dimensions, Distance, Point, Transformable, Vector};

/// A [Rect]angle
///
/// # Diagonals
///
/// [Rect] is constructed from (and represented by) a [Vector] representing any diagonal of the rectangle. Internally it always stores top-left to bottom-right.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect(Vector);

impl Rect {
    /// Construct from the [Point::ORIGIN] and [Dimensions]
    pub fn from_origin_with_dimensions(dims: Dimensions) -> Self {
        Self::from_top_left_and_dimensions(Point::ORIGIN, dims)
    }

    /// Construct from the top-left point and [Dimensions]
    pub fn from_top_left_and_dimensions(top_left: Point, dims: Dimensions) -> Self {
        Self::from_diagonal(top_left.with_delta(dims.into_bottom_right()))
    }

    /// Construct from (either) diagonal [Vector]
    pub fn from_diagonal(diag: Vector) -> Self {
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

    /// The inner circle tangent to the closest sides
    pub fn inner_circle(self) -> Circle {
        self.center().with_radius(self.inner_radius())
    }

    /// The inner radius is half the minimum side length
    pub fn inner_radius(self) -> Distance {
        self.minimum_side_length() * Norm::HALF
    }

    /// The minimum side length
    pub fn minimum_side_length(self) -> Distance {
        let Dimensions { width, height } = self.dimensions();
        width.min(height) * Norm::HALF
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
