use test_case::test_case;

use crate::Direction::{self, *};
use crate::{Bounds, Coord, GridCoord};

#[test_case((0, 0), North => (0, 2))]
#[test_case((0, 0), NorthEast => (1, 2))]
#[test_case((0, 0), East => (1, 0))]
#[test_case((0, 0), SouthEast => (1, 1))]
#[test_case((0, 0), South => (0, 1))]
#[test_case((0, 0), SouthWest => (2, 1))]
#[test_case((0, 0), West => (2, 0))]
#[test_case((0, 0), NorthWest => (2, 2))]
#[test_case((1, 1), North => (1, 0))]
#[test_case((1, 1), NorthEast => (2, 0))]
#[test_case((1, 1), East => (2, 1))]
#[test_case((1, 1), SouthEast => (2, 2))]
#[test_case((1, 1), South => (1, 2))]
#[test_case((1, 1), SouthWest => (0, 2))]
#[test_case((1, 1), West => (0, 1))]
#[test_case((1, 1), NorthWest => (0, 0))]
fn in_3x3_step(xy: (usize, usize), dir: Direction) -> (usize, usize) {
    let pt = GridCoord::new(Coord::from(xy), Bounds::new(3, 3));
    (pt + dir).point().into()
}
