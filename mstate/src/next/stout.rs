//! `* -> (S, O)`: The [Stout] `Next` type and associated [TakeIntoStout] and [IntoStout] traits

use derive_more::{From, Into};
use derive_new::new;

use crate::next::{MapState, State};

/// `(S, O)`
///
/// A container for a state `S` and an output `O`
#[derive(Copy, Clone, Debug, From, Into, new)]
pub struct Stout<S, O> {
    /// The next state `S`
    pub state: S,
    /// The next output, `O`
    pub output: O,
}

impl<S> From<State<S>> for Stout<S, ()> {
    fn from(State { state }: State<S>) -> Self {
        Self::new(state, ())
    }
}

impl<S, O> MapState<S> for Stout<S, O> {
    type MappedState<MS> = Stout<MS, O>;

    fn map_state<F, T>(self, f: F) -> Stout<T, O>
    where
        F: FnOnce(S) -> T,
    {
        Stout {
            state: f(self.state),
            output: self.output,
        }
    }
}
