use antbox_geom::Grid;
use derive_more::{AsRef, Deref, DerefMut, From, Into};
use derive_new::new;
use movestate::into::IntoNextWith;

use crate::{ConwayCell, ConwayGrid};

/// A wrapped [ConwayGrid] which implements [IntoNext](movestate::into::IntoNext) for Conway iterations
#[derive(Debug, Deref, DerefMut, AsRef, From, Into, new)]
pub struct ConwayMachine<C>(Grid<C>)
where
    C: ConwayCell;

impl<C> IntoNextWith<()> for ConwayMachine<C>
where
    C: ConwayCell,
{
    type Next = Self;

    fn into_next_with(self, (): ()) -> Self::Next {
        ConwayMachine(self.0.conway_step())
    }
}
