//! An abstract cellular automata engine
#![deny(unsafe_code, missing_docs)]

use antbox_geom::Grid;

/// An implementation of Conway's Life
#[derive(Debug)]
pub struct ConwaysLife {
    grid: Grid<Cell>,
}

impl ConwaysLife {
    /// Return a [Grid] of neighbor counts
    pub fn count_neighbors(&self) -> Grid<u8> {
        let mut neighbors = Grid::from(self.grid.bounds());
        for pt in neighbors.bounds().iter_points() {
            for npt in pt.neighbors() {
                if self.grid[npt].is_alive() {
                    neighbors[pt] += 1;
                }
            }
        }
        neighbors
    }
}

#[derive(Copy, Clone, Debug)]
struct Cell(bool);

impl Cell {
    fn is_alive(self) -> bool {
        self.0
    }
}
