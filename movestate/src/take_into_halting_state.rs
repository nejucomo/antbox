use crate::TakeIntoNext;
use crate::next::{Halting, State};

/// `(S, I) -> S`
pub trait TakeIntoHaltingState<I>:
    Sized + TakeIntoNext<I, Next: Into<Halting<State<Self>>>>
{
    /// `(S, I) -> S`
    fn take_into_opt_self(self, input: I) -> Option<Self> {
        self.take_into_hstate(input).into()
    }

    /// Transition `self` and an `input` into a [Halting] [State]
    ///
    /// Consumers typically use [TakeIntoHaltingState::take_into_opt_self] for ergonomics.
    fn take_into_hstate(self, input: I) -> Halting<State<Self>> {
        self.take_into_next(input).into()
    }
}

impl<B, I> TakeIntoHaltingState<I> for B where
    B: Sized + TakeIntoNext<I, Next: Into<Halting<State<Self>>>>
{
}
