use moveslot::{MapInPlace as _, MoveSlot};

use crate::TakeIntoNext;
use crate::starg::Starg;

/// Mutably update and input `I` to produce an `O`
pub trait Update<I, O> {
    /// Mutable update with `input` to produce a `O`
    fn update(&mut self, input: I) -> O;
}

impl<S, I, O> Update<I, O> for MoveSlot<S>
where
    S: TakeIntoNext<I, Next: Into<Starg<S, O>>>,
{
    fn update(&mut self, input: I) -> O {
        self.mapout_in_place(|s| {
            let Starg { state, arg } = s.take_into_next(input).into();
            (state, arg)
        })
    }
}
