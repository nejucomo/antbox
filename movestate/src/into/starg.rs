use crate::into::IntoNext;
use crate::starg::Starg;
use crate::take_into::TakeIntoStarg;

/// `S -> (S, O)`: Transition `self` into a next state and value via [Starg], aka an _endless sequence_
pub trait IntoStarg<O>: IntoNext + TakeIntoStarg<(), O> {
    /// Convert `self` into a next state and output
    fn into_starg(self) -> Starg<Self, O> {
        self.into_next().into()
    }
}

impl<B, O> IntoStarg<O> for B where B: IntoNext + TakeIntoStarg<(), O> {}
