use std::ops::Add;

use derive_more::{From, Into};
use derive_new::new;

use crate::{Bounds, Direction, Point};

/// A [Point] in the context
#[derive(Copy, Clone, Debug, From, Into, new)]
pub struct BoundPoint {
    #[new(into)]
    pt: Point,
    #[new(into)]
    bounds: Bounds,
}

impl BoundPoint {
    /// The raw [Point]
    pub fn point(self) -> Point {
        self.pt
    }

    /// The [Bounds]
    pub fn bounds(self) -> Bounds {
        self.bounds
    }

    /// The x coordinate
    pub fn x(self) -> usize {
        self.pt.x
    }

    /// The y coordinate
    pub fn y(self) -> usize {
        self.pt.y
    }

    /// The width bound
    pub fn width(self) -> usize {
        self.bounds.width
    }

    /// The height bound
    pub fn height(self) -> usize {
        self.bounds.height
    }

    /// The (wrap-around) neighbor [BoundPoint]s
    pub fn neighbors(self) -> impl Iterator<Item = BoundPoint> {
        Direction::each().map(move |d| self + d)
    }
}

impl From<BoundPoint> for usize {
    fn from(bp: BoundPoint) -> Self {
        bp.pt.y * bp.bounds.width + bp.pt.x
    }
}

impl Add<Direction> for BoundPoint {
    type Output = BoundPoint;

    fn add(self, dir: Direction) -> Self::Output {
        let BoundPoint {
            pt: Point { x, y },
            bounds: Bounds { width, height },
        } = self;

        // TODO: The lesson is to hardcode `isize` or `usize`, not `usize` in the geometry types.
        let (dx, dy) = dir.wrap_around_deltas(width, height);
        let nx = (x + dx) % width;
        let ny = (y + dy) % height;

        BoundPoint::new((nx, ny), self.bounds)
    }
}
