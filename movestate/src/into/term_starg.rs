use crate::into::IntoNext;
use crate::starg::TermStarg;
use crate::take_into::TakeIntoTermStarg;

/// `S -> Either<(S, O), T>`: Transition `self` into a new state/output `(S, O)` or else a terminal `T` value
pub trait IntoTermStarg<O, T>: IntoNext + TakeIntoTermStarg<(), O, T> {
    /// Convert `self` into a next state/output or else a terminal value
    fn into_term_starg(self) -> TermStarg<Self, O, T> {
        self.into_next().into()
    }
}
