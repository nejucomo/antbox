/// A Conway's Life [Cell]
#[derive(Copy, Clone, Default, Debug)]
pub struct Cell {
    alive: bool,
    neighbors: u8,
}

impl Cell {
    /// Is this [Cell] alive?
    pub fn is_alive(self) -> bool {
        self.alive
    }

    /// Is this [Cell] alive?
    pub fn set_alive(&mut self, alive: bool) {
        self.alive = alive;
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
