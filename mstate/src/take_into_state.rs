use crate::TakeIntoNext;
use crate::next::State;

/// `(S, I) -> S`
pub trait TakeIntoState<I>: Sized + TakeIntoNext<I, Next: Into<State<Self>>> {
    /// `(S, I) -> S`
    fn take_into_self(self, input: I) -> Self {
        self.take_into_state(input).state
    }

    /// This transitions `self` and `input` into a [State]
    ///
    /// Consumers typically use [TakeIntoState::take_into_self] for ergonomics.
    fn take_into_state(self, input: I) -> State<Self> {
        self.take_into_next(input).into()
    }
}

/// Blanket Extension from [TakeIntoNext]
impl<B, I> TakeIntoState<I> for B where B: Sized + TakeIntoNext<I, Next: Into<State<B>>> {}
