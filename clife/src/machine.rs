use antbox_grid::Grid;
use derive_more::{AsRef, Deref, DerefMut, From, Into};
use derive_new::new;
use mstate::MStateIn;

use crate::{ConwayCell, ConwayGrid};

/// A wrapped [ConwayGrid] which implements [MStateIn] for Conway iterations
#[derive(Debug, Deref, DerefMut, AsRef, From, Into, new)]
pub struct ConwayMachine<C>(Grid<C>)
where
    C: ConwayCell;

impl<C> MStateIn<()> for ConwayMachine<C>
where
    C: ConwayCell,
{
    type Next = Self;

    fn into_with(self, (): ()) -> Self {
        ConwayMachine(self.0.conway_step())
    }
}
