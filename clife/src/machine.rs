use antbox_geom::Grid;
use derive_more::{AsRef, Deref, DerefMut, From, Into};
use derive_new::new;
use movestate::IntoNext;

use crate::{ConwayCell, ConwayGrid};

/// A wrapped [ConwayGrid] which implements [IntoNext] for Conway iterations
#[derive(Debug, Deref, DerefMut, AsRef, From, Into, new)]
pub struct ConwayMachine<C>(Grid<C>)
where
    C: ConwayCell;

impl<C> IntoNext for ConwayMachine<C>
where
    C: ConwayCell,
{
    fn into_next(self) -> Self {
        ConwayMachine(self.0.conway_step())
    }
}
