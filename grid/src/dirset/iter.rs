use derive_new::new;

use crate::{DirIter, DirSet, Direction};

/// An iterator over the [Direction]s within a [DirSet]
#[derive(Copy, Clone, Debug, new)]
pub struct DirSetIter {
    dirs: DirSet,
    #[new(default)]
    it: DirIter,
}

impl Iterator for DirSetIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.find(|&d| self.dirs.has(d))
    }
}
