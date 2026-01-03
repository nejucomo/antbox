use crate::next::{Halting, Stout};
use crate::stditer::IHSIter;
use crate::{IntoNext, TakeIntoHaltingStout};

/// `S -> [S, O]`; isomorphic conversions to/from [Iterator]
pub trait IntoHaltingStout<O>: IntoNext + TakeIntoHaltingStout<(), O> {
    /// `S -> [S, O]`
    fn into_opt_self_out(self) -> Option<(Self, O)> {
        self.into_hstout().into()
    }

    /// Transition `self` into a [Halting] [Stout]
    ///
    /// Consumers typically use [IntoHaltingStout::into_opt_self_out] for ergonomics.
    fn into_hstout(self) -> Halting<Stout<Self, O>> {
        self.into_next().into()
    }

    /// Convert `self` into an [Iterator]
    fn into_iterator(self) -> IHSIter<Self, O> {
        IHSIter::new(self)
    }
}

impl<B, O> IntoHaltingStout<O> for B where B: IntoNext + TakeIntoHaltingStout<(), O> {}
