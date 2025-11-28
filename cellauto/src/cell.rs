/// A Conway's Life [Cell]
#[derive(Copy, Clone, Default, Debug)]
pub struct Cell {
    life: bool,
    neighbors: u8,
}

impl Cell {
    /// Is this [Cell] alive?
    pub fn is_alive(self) -> bool {
        self.life
    }

    /// The number of neighbors
    pub fn neighbor_count(self) -> u8 {
        self.neighbors
    }

    /// Increment the number of neighbors
    pub(crate) fn count_neighbor(&mut self) {
        self.neighbors += 1
    }
}
