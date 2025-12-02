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
    pub fn deltas(self) -> (isize, isize) {
        match self {
            North => (0, -1),
            NorthEast => (1, -1),
            East => (1, 0),
            SouthEast => (1, 1),
            South => (0, 1),
            SouthWest => (-1, 1),
            West => (-1, 0),
            NorthWest => (-1, -1),
        }
    }
}
