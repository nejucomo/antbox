use antbox_grid::Grid;
use derive_more::{AsRef, Deref, DerefMut, From, Into};
use derive_new::new;
use mstate::TakeIntoNext;

use crate::{ConwayCell, ConwayGrid};

/// A wrapped [ConwayGrid] which implements [IntoNext](mstate::IntoNext) for Conway iterations
#[derive(Debug, Deref, DerefMut, AsRef, From, Into, new)]
pub struct ConwayMachine<C>(Grid<C>)
where
    C: ConwayCell;

impl<C> TakeIntoNext<()> for ConwayMachine<C>
where
    C: ConwayCell,
{
    type Next = Self;

    fn take_into_next(self, (): ()) -> Self::Next {
        ConwayMachine(self.0.conway_step())
    }
}
