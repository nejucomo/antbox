use crate::UpdateInput;

/// A type which can functionally update (ie: with move-semantics) given an `Input` to a new state and while producing an [Output](UpdateIO::Output)
pub trait UpdateIO<Input>: Sized {
    /// The output of a transform
    type Output;

    /// Transform the `self` state with `i` into a new [Self] state and an [Output](Self::Output)
    fn update_io(self, i: Input) -> (Self, Self::Output);
}

impl<T, I> UpdateIO<I> for T
where
    T: UpdateInput<I>,
{
    type Output = ();

    fn update_io(self, i: I) -> (Self, Self::Output) {
        (self.update_input(i), ())
    }
}
