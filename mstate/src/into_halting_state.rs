use crate::next::{Halting, State};
use crate::{IntoNext, TakeIntoNext};

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

impl<B> IntoHaltingState for B where B: IntoNext + TakeIntoNext<(), Next: Into<Halting<State<Self>>>>
{}
