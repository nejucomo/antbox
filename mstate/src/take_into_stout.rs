use crate::TakeIntoNext;
use crate::next::Stout;

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

impl<B, I, O> TakeIntoStout<I, O> for B where B: Sized + TakeIntoNext<I, Next: Into<Stout<B, O>>> {}
