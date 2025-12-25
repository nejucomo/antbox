//! `* -> S`: The [State] `Next` type and associated [TakeIntoState] and [IntoState] traits
mod fromimpls;

use derive_more::From;
use derive_new::new;

use crate::{IntoNext, TakeIntoNext};

/// `S`
///
/// A newtype wrapper around `S` primarily used by producer code to facilitate conversions and blanket extensions
#[derive(Copy, Clone, Debug, From, new)]
pub struct State<S> {
    /// The wrapped next state, `S`
    pub state: S,
}

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

mod blanket_extensions {
    use crate::{IntoNext, TakeIntoNext};

    use super::{IntoState, State, TakeIntoState};

    impl<B, I> TakeIntoState<I> for B where B: Sized + TakeIntoNext<I, Next: Into<State<B>>> {}

    impl<B> IntoState for B where B: IntoNext + TakeIntoState<()> {}
}
