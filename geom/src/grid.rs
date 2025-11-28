use std::ops::{Index, IndexMut};

use try_from_unwrap::TryFromUnwrap as _;

use crate::{BoundPoint, Bounds};

/// A 2-D grid of `T` cells
#[derive(Debug)]
pub struct Grid<T> {
    bounds: Bounds<u32>,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    /// The bounds of this [Grid]
    pub fn bounds(&self) -> Bounds<u32> {
        self.bounds
    }
}

impl<T> From<Bounds<u32>> for Grid<T>
where
    T: Default + Clone,
{
    fn from(bounds: Bounds<u32>) -> Self {
        Grid {
            bounds,
            cells: vec![T::default(); usize::tfu(bounds.area())],
        }
    }
}

impl<T> Index<BoundPoint<u32>> for Grid<T> {
    type Output = T;

    fn index(&self, bp: BoundPoint<u32>) -> &Self::Output {
        &self.cells[usize::from(bp)]
    }
}

impl<T> IndexMut<BoundPoint<u32>> for Grid<T> {
    fn index_mut(&mut self, bp: BoundPoint<u32>) -> &mut Self::Output {
        &mut self.cells[usize::from(bp)]
    }
}
