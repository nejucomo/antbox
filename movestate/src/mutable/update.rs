use moveslot::{MapInPlace as _, MoveSlot};

use crate::TakeIntoStout;

/// Mutable update with input `I` to produce an `O`
///
/// # TODO
///
/// - Make `O` an associated parameter and pivot all of `movestate` to match.
pub trait Update<I, O> {
    /// Mutable update with `input` to produce an `O`
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
