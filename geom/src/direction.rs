use Direction::*;

/// An 8-point neighbor direction for a cartesian grid
#[derive(Copy, Clone, Debug)]
#[allow(missing_docs)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    /// Iterator over each [Direction]
    pub fn each() -> impl Iterator<Item = Direction> {
        [
            North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
        ]
        .into_iter()
    }

    /// Return the coordinate deltas for this direction
    pub fn wrap_around_deltas(self, width: usize, height: usize) -> (usize, usize) {
        match self {
            North => (0, height - 1),
            NorthEast => (1, height - 1),
            East => (1, 0),
            SouthEast => (1, 1),
            South => (0, 1),
            SouthWest => (width - 1, 1),
            West => (width - 1, 0),
            NorthWest => (width - 1, height - 1),
        }
    }
}
