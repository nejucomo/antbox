use crate::into::IntoNext;
use crate::starg::Starg;
use crate::take_into::TakeIntoOptStarg;

/// `S -> Option<(S, O)>`: Transition `self` into a new state/output `(S, O)` or [None]
pub trait IntoOptStarg<O>: IntoNext + TakeIntoOptStarg<(), O> {
    /// Convert `self` into a next state/output or [None]
    fn into_opt_starg(self) -> Option<Starg<Self, O>> {
        self.into_next().into()
    }
}

impl<B, O> IntoOptStarg<O> for B where B: IntoNext + TakeIntoOptStarg<(), O> {}
