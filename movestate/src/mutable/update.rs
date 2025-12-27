use moveslot::{MapInPlace as _, MoveSlot};

use crate::TakeIntoStout;

/// Mutable update with input `I` to produce a `O`
pub trait Update<I, O> {
    /// Mutable update with `input` to produce a `O`
    fn update(&mut self, input: I) -> O;
}

impl<S, I, O> Update<I, O> for MoveSlot<S>
where
    S: TakeIntoStout<I, O>,
{
    fn update(&mut self, input: I) -> O {
        self.mip_out(|s| s.take_into_self_out(input))
    }
}
