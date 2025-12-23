use crate::{IntoNext, Starg, TakeIntoStarg};

/// `S -> (S, O)`: Transition `self` into a next state and value via [Starg], aka an _endless sequence_
pub trait IntoStarg<O>: IntoNext + TakeIntoStarg<(), O> {
    /// Convert `self` into a next state and output
    fn into_starg(self) -> Starg<Self, O> {
        self.into_next().into()
    }
}
