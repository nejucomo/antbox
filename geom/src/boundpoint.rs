use derive_more::{From, Into};
use derive_new::new;
use itertools::Itertools as _;
use try_from_unwrap::TryFromUnwrap as _;

use crate::{Bounds, Point};

/// A [Point] in the context
#[derive(Copy, Clone, Debug, From, Into, new)]
pub struct BoundPoint {
    pt: Point,
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
        let w = isize::tfu(self.bounds().width);
        let h = isize::tfu(self.bounds().height);

        (-1isize..=1)
            .cartesian_product(-1isize..=1)
            .filter_map(move |(dx, dy)| {
                if dx == 0 && dy == 0 {
                    None
                } else {
                    Some(((w + dx) % w, (h + dy) % h))
                }
            })
            .map(|(x, y)| Point::new(u32::tfu(x), u32::tfu(y)))
            .map(move |pt| BoundPoint::new(pt, self.bounds))
    }
}

impl From<BoundPoint> for usize {
    fn from(bp: BoundPoint) -> Self {
        usize::tfu(bp.pt.y * bp.bounds.width + bp.pt.x)
    }
}
