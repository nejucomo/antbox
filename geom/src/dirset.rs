mod iter;

use rand::Rng;
use rand::distr::Distribution;
use rand::seq::IteratorRandom as _;

use crate::Direction;

pub use self::iter::DirSetIter;

/// A set of [Direction]s
#[derive(Copy, Clone, Debug, Default)]
pub struct DirSet(u8);

impl DirSet {
    /// If there are no [Direction]s in this set
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// If `d` is in this set
    pub fn has(self, d: Direction) -> bool {
        self.0 & d.mask() != 0
    }

    /// The `DirSet` with `self` also including `d`
    pub fn with(self, d: Direction) -> Self {
        DirSet(self.0 | d.mask())
    }

    /// The intersection with `other`
    pub fn intersect(self, other: DirSet) -> Self {
        DirSet(self.0 | other.0)
    }
}

impl IntoIterator for DirSet {
    type Item = Direction;
    type IntoIter = DirSetIter;

    fn into_iter(self) -> Self::IntoIter {
        DirSetIter::new(self)
    }
}

impl Distribution<Option<Direction>> for DirSet {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<Direction> {
        self.into_iter().choose(rng)
    }
}
