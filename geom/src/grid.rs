use std::ops::{Index, IndexMut};

use try_from_unwrap::TryFromUnwrap as _;

use crate::{BoundPoint, Bounds, Point};

/// A 2-D grid of `T` cells
#[derive(Clone, PartialEq)]
pub struct Grid<T> {
    bounds: Bounds,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    /// Create a new grid from constituent parts
    ///
    /// # Panics
    ///
    /// If `bounds.area() != cells.len()` this panics
    pub fn new(bounds: Bounds, cells: Vec<T>) -> Self {
        assert_eq!(usize::tfu(bounds.area()), cells.len());
        Grid { bounds, cells }
    }

    /// The bounds of this [Grid]
    pub fn bounds(&self) -> Bounds {
        self.bounds
    }

    /// Iterate over `(pt, &T)`
    pub fn iter(&self) -> impl Iterator<Item = (BoundPoint, &T)> {
        self.cells
            .iter()
            .enumerate()
            .map(|(ix, cptr)| (self.bounds.ix_to_bp(ix), cptr))
    }

    /// Iterate over `(pt, &mut T)`
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (BoundPoint, &mut T)> {
        self.cells
            .iter_mut()
            .enumerate()
            .map(|(ix, cptr)| (self.bounds.ix_to_bp(ix), cptr))
    }

    /// Map cells
    pub fn map_cell_refs<F, U>(&self, f: F) -> Grid<U>
    where
        F: Fn(BoundPoint, &T) -> U,
    {
        Grid {
            bounds: self.bounds(),
            cells: self.iter().map(|(bpt, c)| f(bpt, c)).collect(),
        }
    }
}

impl<T> From<Bounds> for Grid<T>
where
    T: Default + Clone,
{
    fn from(bounds: Bounds) -> Self {
        Grid {
            bounds,
            cells: vec![T::default(); usize::tfu(bounds.area())],
        }
    }
}

impl<T> Index<BoundPoint> for Grid<T> {
    type Output = T;

    fn index(&self, bp: BoundPoint) -> &Self::Output {
        &self.cells[usize::from(bp)]
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, p: Point) -> &Self::Output {
        &self[BoundPoint::new(p, self.bounds())]
    }
}

impl<T> Index<(u32, u32)> for Grid<T> {
    type Output = T;

    fn index(&self, p: (u32, u32)) -> &Self::Output {
        &self[Point::from(p)]
    }
}

impl<T> IndexMut<BoundPoint> for Grid<T> {
    fn index_mut(&mut self, bp: BoundPoint) -> &mut Self::Output {
        &mut self.cells[usize::from(bp)]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, p: Point) -> &mut Self::Output {
        let bounds = self.bounds();
        &mut self[BoundPoint::new(p, bounds)]
    }
}

impl<T> IndexMut<(u32, u32)> for Grid<T> {
    fn index_mut(&mut self, p: (u32, u32)) -> &mut Self::Output {
        &mut self[Point::from(p)]
    }
}

impl<T> std::fmt::Debug for Grid<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid [")?;
        for row in self.cells.chunks_exact(usize::tfu(self.bounds.width)) {
            write!(f, "  ")?;
            for t in row {
                write!(f, "{t:?}")?;
            }
            writeln!(f)?;
        }
        writeln!(f, "]")
    }
}
