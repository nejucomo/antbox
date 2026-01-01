use std::ops::Add;

use derive_more::{From, Into};
use derive_new::new;

use crate::{Bounds, Coord, Direction};

/// A [Coord] within some [Bounds]
#[derive(Copy, Clone, From, Into, new, Eq, Ord, PartialEq, PartialOrd)]
pub struct GridCoord {
    #[new(into)]
    pt: Coord,
    #[new(into)]
    bounds: Bounds,
}

impl GridCoord {
    /// The raw [Coord]
    pub fn point(self) -> Coord {
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

    /// The (wrap-around) neighbor [GridCoord]s
    pub fn neighbors(self) -> impl Iterator<Item = GridCoord> {
        Direction::each().map(move |d| self + d)
    }
}

impl From<GridCoord> for usize {
    fn from(bp: GridCoord) -> Self {
        bp.pt.y * bp.bounds.width + bp.pt.x
    }
}

impl Add<Direction> for GridCoord {
    type Output = GridCoord;

    fn add(self, dir: Direction) -> Self::Output {
        let GridCoord {
            pt: Coord { x, y },
            bounds: Bounds { width, height },
        } = self;

        let (dx, dy) = dir.wrap_around_deltas(width, height);
        let nx = (x + dx) % width;
        let ny = (y + dy) % height;

        GridCoord::new((nx, ny), self.bounds)
    }
}

impl std::fmt::Debug for GridCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let GridCoord { pt, bounds } = self;
        write!(f, "{pt:?} {bounds:?}")
    }
}
