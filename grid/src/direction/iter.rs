use crate::Direction::{self, North};

/// An iterator over [Direction]s
#[derive(Copy, Clone, Debug)]
pub struct DirIter(Option<Direction>);

impl Default for DirIter {
    fn default() -> Self {
        Self(Some(North))
    }
}

impl Iterator for DirIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().inspect(|d| {
            let cw = d.clockwise();
            if !matches!(cw, North) {
                self.0 = Some(cw);
            }
        })
    }
}
