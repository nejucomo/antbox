//! `* -> (S, O)`: The [Stout] `Next` type and associated [TakeIntoStout] and [IntoStout] traits
mod fromimpls;

use derive_more::{From, Into};
use derive_new::new;

use crate::{IntoNext, MapState, TakeIntoNext};

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

impl<S, O> MapState<S> for Stout<S, O> {
    type MappedState<MS> = Stout<MS, O>;

    fn map_state<F, T>(self, f: F) -> Self::MappedState<T>
    where
        F: FnOnce(S) -> T,
    {
        Stout {
            state: f(self.state),
            output: self.output,
        }
    }
}

/// `(S, I) -> (S, O)`
pub trait TakeIntoStout<I, O>: Sized + TakeIntoNext<I, Next: Into<Stout<Self, O>>> {
    /// `(S, I) -> (S, O)`
    fn take_into_self_out(self, input: I) -> (Self, O) {
        self.take_into_stout(input).into()
    }

    /// Transition `self` and an `input` into a [Stout]
    ///
    /// Consumers typically use [TakeIntoStout::take_into_self_out] for ergonomics.
    fn take_into_stout(self, input: I) -> Stout<Self, O> {
        self.take_into_next(input).into()
    }
}

/// `S -> (S, O)`
pub trait IntoStout<O>: IntoNext + TakeIntoStout<(), O> {
    /// `S -> (S, O)`
    fn into_self_out(self) -> (Self, O) {
        self.into_stout().into()
    }

    /// Transition `self` into a [Stout]
    ///
    /// Consumers typically use [IntoStout::into_self_out] for ergonomics.
    fn into_stout(self) -> Stout<Self, O> {
        self.into_next().into()
    }
}

mod blanket_extensions {
    use crate::{IntoNext, TakeIntoNext};

    use super::{IntoStout, Stout, TakeIntoStout};

    impl<B, I, O> TakeIntoStout<I, O> for B where B: Sized + TakeIntoNext<I, Next: Into<Stout<B, O>>> {}

    impl<B, O> IntoStout<O> for B where B: IntoNext + TakeIntoStout<(), O> {}
}
