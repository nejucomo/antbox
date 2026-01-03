//! `* -> S`: The [State] `Next` type and associated [TakeIntoState] and [IntoState] traits
use derive_more::From;
use derive_new::new;

use crate::next::{MapState, Stout};

/// `S`
///
/// A newtype wrapper around `S` primarily used by producer code to facilitate conversions and blanket extensions
#[derive(Copy, Clone, Debug, From, new)]
pub struct State<S> {
    /// The wrapped next state, `S`
    pub state: S,
}

impl<S> From<Stout<S, ()>> for State<S> {
    fn from(Stout { state, output: _ }: Stout<S, ()>) -> Self {
        Self::new(state)
    }
}

impl<S> MapState<S> for State<S> {
    type MappedState<MS> = State<MS>;

    fn map_state<F, T>(self, f: F) -> State<T>
    where
        F: FnOnce(S) -> T,
    {
        State {
            state: f(self.state),
        }
    }
}
