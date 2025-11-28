use std::ops::{Index, IndexMut};

use try_from_unwrap::TryFromUnwrap as _;

use crate::{BoundPoint, Bounds};

/// A 2-D grid of `T` cells
#[derive(Debug)]
pub struct Grid<T> {
    bounds: Bounds,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    /// The bounds of this [Grid]
    pub fn bounds(&self) -> Bounds {
        self.bounds
    }

    /// Iterate over `(pt, &mut T)`
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (BoundPoint, &mut T)> {
        self.cells
            .iter_mut()
            .enumerate()
            .map(|(ix, cptr)| (self.bounds.ix_to_bp(ix), cptr))
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

impl<T> IndexMut<BoundPoint> for Grid<T> {
    fn index_mut(&mut self, bp: BoundPoint) -> &mut Self::Output {
        &mut self.cells[usize::from(bp)]
    }
}
