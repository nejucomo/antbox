use crate::next::Stout;
use crate::{IntoNext, TakeIntoStout};

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

impl<B, O> IntoStout<O> for B where B: IntoNext + TakeIntoStout<(), O> {}
