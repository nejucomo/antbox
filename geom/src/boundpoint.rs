use std::ops::Add;

use derive_more::{From, Into};
use derive_new::new;
use try_from_unwrap::TryFromUnwrap as _;

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
    pub fn x(self) -> u32 {
        self.pt.x
    }

    /// The y coordinate
    pub fn y(self) -> u32 {
        self.pt.y
    }

    /// The width bound
    pub fn width(self) -> u32 {
        self.bounds.width
    }

    /// The height bound
    pub fn height(self) -> u32 {
        self.bounds.height
    }

    /// The (wrap-around) neighbor [BoundPoint]s
    pub fn neighbors(self) -> impl Iterator<Item = BoundPoint> {
        Direction::each().map(move |d| self + d)
    }
}

impl From<BoundPoint> for usize {
    fn from(bp: BoundPoint) -> Self {
        usize::tfu(bp.pt.y * bp.bounds.width + bp.pt.x)
    }
}

impl Add<Direction> for BoundPoint {
    type Output = BoundPoint;

    fn add(self, dir: Direction) -> Self::Output {
        let BoundPoint {
            pt: Point { x, y },
            bounds: Bounds { width, height },
        } = self;

        // TODO: The lesson is to hardcode `isize` or `usize`, not `u32` in the geometry types.
        let x = isize::tfu(x);
        let y = isize::tfu(y);
        let width = isize::tfu(width);
        let height = isize::tfu(height);

        let (dx, dy) = dir.deltas();
        let nx = (x + width + dx) % width;
        let ny = (y + height + dy) % height;

        BoundPoint::new((u32::tfu(nx), u32::tfu(ny)), self.bounds)
    }
}
