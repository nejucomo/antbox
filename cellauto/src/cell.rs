use derive_more::{From, Into};

/// A Conway's Life [Cell]
#[derive(Copy, Clone, Default, From, Into, PartialEq)]
pub struct Cell(bool);

impl Cell {
    /// Is this [Cell] alive?
    pub fn is_alive(self) -> bool {
        self.0
    }

    /// Is this [Cell] alive?
    pub fn set_alive(&mut self, alive: bool) {
        self.0 = alive;
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.0 { '⏺' } else { '•' })
    }
}
