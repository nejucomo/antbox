use crate::TakeIntoNext;
use crate::next::{Halting, Stout};

/// `(S, I) -> [S, O]`
pub trait TakeIntoHaltingStout<I, O>:
    Sized + TakeIntoNext<I, Next: Into<Halting<Stout<Self, O>>>>
{
    /// `(S, I) -> [S, O]`
    fn take_into_opt_self_out(self, input: I) -> Option<(Self, O)> {
        self.take_into_hstout(input).into()
    }

    /// Transition `self` and an `input` into a [Halting] [Stout]
    ///
    /// Consumers typically use [TakeIntoHaltingStout::take_into_opt_self_out] for ergonomics.
    fn take_into_hstout(self, input: I) -> Halting<Stout<Self, O>> {
        self.take_into_next(input).into()
    }
}

impl<B, I, O> TakeIntoHaltingStout<I, O> for B where
    B: Sized + TakeIntoNext<I, Next: Into<Halting<Stout<Self, O>>>>
{
}
