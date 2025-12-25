use crate::halting::Halting;
use crate::state::State;
use crate::{IntoNext, TakeIntoNext};

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

/// `S -> S`
pub trait IntoHaltingState: IntoNext + TakeIntoNext<(), Next: Into<Halting<State<Self>>>> {
    /// `S -> S`
    fn into_opt_self(self) -> Option<Self> {
        self.into_hstate().into()
    }

    /// Transition `self` into a [Halting] [State]
    ///
    /// Consumers typically use [IntoHaltingState::into_opt_self] for ergonomics.
    fn into_hstate(self) -> Halting<State<Self>> {
        self.into_next().into()
    }
}

mod blanket_extensions {
    use crate::halting::{Halting, IntoHaltingState, TakeIntoHaltingState};
    use crate::state::State;
    use crate::{IntoNext, TakeIntoNext};

    impl<B, I> TakeIntoHaltingState<I> for B where
        B: Sized + TakeIntoNext<I, Next: Into<Halting<State<Self>>>>
    {
    }

    impl<B> IntoHaltingState for B where B: IntoNext + TakeIntoNext<(), Next: Into<Halting<State<Self>>>>
    {}
}
