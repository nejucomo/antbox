mod iter;

use Direction::*;
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};

pub use self::iter::DirIter;

/// An 8-point neighbor direction for a cartesian grid
#[derive(Copy, Clone, Debug)]
#[allow(missing_docs)]
#[repr(u8)]
pub enum Direction {
    North = 0,
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
        DirIter::default()
    }

    /// The clockwise direction
    pub fn clockwise(self) -> Direction {
        match self {
            North => NorthEast,
            NorthEast => East,
            East => SouthEast,
            SouthEast => South,
            South => SouthWest,
            SouthWest => West,
            West => NorthWest,
            NorthWest => North,
        }
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

    pub(crate) fn mask(self) -> u8 {
        1 << (self as u8)
    }
}

impl Distribution<Direction> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.random_range(0..8) {
            0 => North,
            1 => NorthEast,
            2 => East,
            3 => SouthEast,
            4 => South,
            5 => SouthWest,
            6 => West,
            7 => NorthWest,
            _ => unreachable!(),
        }
    }
}
