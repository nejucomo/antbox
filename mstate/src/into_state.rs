use crate::next::State;
use crate::{IntoNext, TakeIntoState};

/// `S -> S`
pub trait IntoState: IntoNext + TakeIntoState<()> {
    /// `S -> S`
    fn into_self(self) -> Self {
        self.into_state().state
    }

    /// This transitions `self` and `input` into a [State]
    ///
    /// Consumers typically use [IntoState::into_self] for ergonomics.
    fn into_state(self) -> State<Self> {
        self.into_next().into()
    }
}

impl<B> IntoState for B where B: IntoNext + TakeIntoState<()> {}
