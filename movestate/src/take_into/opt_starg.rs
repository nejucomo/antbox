use crate::starg::Starg;
use crate::take_into::TakeIntoNext;

/// `(S, I) -> Option<(S, O)>`: Take an input into a new state/output `(S, O)` or [None]
pub trait TakeIntoOptStarg<I, O>:
    Sized + TakeIntoNext<I, Next: Into<Option<Starg<Self, O>>>>
{
    /// Take `self` and `input` into either a new `Starg<Self, O>` or [None]
    fn take_into_opt_starg(self, input: I) -> Option<Starg<Self, O>> {
        self.take_into_next(input).into()
    }
}

impl<B, I, O> TakeIntoOptStarg<I, O> for B where
    B: Sized + TakeIntoNext<I, Next: Into<Option<Starg<Self, O>>>>
{
}
