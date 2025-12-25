use moveslot::{MapInPlace as _, MoveSlot};

use crate::stout::TakeIntoStout;

/// Mutably update and input `I` to produce an `O`
pub trait Update<I, O> {
    /// Mutable update with `input` to produce a `O`
    fn update(&mut self, input: I) -> O;
}

impl<S, I, O> Update<I, O> for MoveSlot<S>
where
    S: TakeIntoStout<I, O>,
{
    fn update(&mut self, input: I) -> O {
        self.mapout_in_place(|s| s.take_into_self_out(input))
    }
}
