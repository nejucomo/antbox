use crate::Starg;
use crate::take_into::TakeIntoNext;

/// `(S, I) -> (S, O)`: Take an input into a new state and output, aka a _Mealy Machine_
pub trait TakeIntoStarg<I, O>: Sized + TakeIntoNext<I, Next: Into<Starg<Self, O>>> {
    /// Take `self` and `input` into a new `Self` and output `O` contained in a [Starg]
    fn take_into_starg(self, input: I) -> Starg<Self, O> {
        self.take_into_next(input).into()
    }
}

impl<B, I, O> TakeIntoStarg<I, O> for B where B: Sized + TakeIntoNext<I, Next: Into<Starg<Self, O>>> {}
