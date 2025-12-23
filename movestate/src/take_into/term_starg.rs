use crate::TermStarg;
use crate::take_into::TakeIntoNext;

/// `(S, I) -> Either<(S, O), T>`: Take an input into a new state/output `(S, O)` or else a terminal `T` value
pub trait TakeIntoTermStarg<I, O, T>:
    Sized + TakeIntoNext<I, Next: Into<TermStarg<Self, O, T>>>
{
    /// Take `self` and `input` into either a new `Starg<Self, O>` or a terminal value, `T`
    fn take_into_term_starg(self, input: I) -> TermStarg<Self, O, T> {
        self.take_into_next(input).into()
    }
}

impl<B, I, O, T> TakeIntoTermStarg<I, O, T> for B where
    B: Sized + TakeIntoNext<I, Next: Into<TermStarg<Self, O, T>>>
{
}
