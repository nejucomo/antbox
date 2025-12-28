use std::ops::{Index, IndexMut};

use crate::{GridCoord, Bounds, Coord, DirSet, Direction};

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
        assert_eq!(bounds.area(), cells.len());
        Grid { bounds, cells }
    }

    /// The bounds of this [Grid]
    pub fn bounds(&self) -> Bounds {
        self.bounds
    }

    /// Iterate over `(pt, &T)`
    pub fn iter(&self) -> impl Iterator<Item = (GridCoord, &T)> {
        self.cells
            .iter()
            .enumerate()
            .map(|(ix, cptr)| (self.bounds.ix_to_bp(ix), cptr))
    }

    /// Iterate over `(pt, &mut T)`
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (GridCoord, &mut T)> {
        self.cells
            .iter_mut()
            .enumerate()
            .map(|(ix, cptr)| (self.bounds.ix_to_bp(ix), cptr))
    }

    /// The directions from `pt` where `f` is true
    pub fn directions_where<F>(&self, pt: GridCoord, mut f: F) -> DirSet
    where
        F: FnMut(&T) -> bool,
    {
        Direction::each()
            .filter(|&d| f(&self[pt + d]))
            .fold(DirSet::default(), DirSet::with)
    }

    /// Map cells
    pub fn map_cell_refs<F, U>(&self, f: F) -> Grid<U>
    where
        F: Fn(GridCoord, &T) -> U,
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
            cells: vec![T::default(); bounds.area()],
        }
    }
}

impl<T> Index<GridCoord> for Grid<T> {
    type Output = T;

    fn index(&self, bp: GridCoord) -> &Self::Output {
        &self.cells[usize::from(bp)]
    }
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, p: Coord) -> &Self::Output {
        &self[GridCoord::new(p, self.bounds())]
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, p: (usize, usize)) -> &Self::Output {
        &self[Coord::from(p)]
    }
}

impl<T> IndexMut<GridCoord> for Grid<T> {
    fn index_mut(&mut self, bp: GridCoord) -> &mut Self::Output {
        &mut self.cells[usize::from(bp)]
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, p: Coord) -> &mut Self::Output {
        let bounds = self.bounds();
        &mut self[GridCoord::new(p, bounds)]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, p: (usize, usize)) -> &mut Self::Output {
        &mut self[Coord::from(p)]
    }
}

impl<T> std::fmt::Debug for Grid<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid [")?;
        for row in self.cells.chunks_exact(self.bounds.width) {
            write!(f, "  ")?;
            for t in row {
                write!(f, "{t:?}")?;
            }
            writeln!(f)?;
        }
        writeln!(f, "]")
    }
}
