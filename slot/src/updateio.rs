/// A type which can functionally update (ie: with move-semantics) given an `Input` to a new state and while producing an [Output](UpdateIO::Output)
pub trait UpdateIO<Input>: Sized {
    /// The output of a transform
    type Output;

    /// Transform the `self` state with `i` into a new [Self] state and an [Output](Self::Output)
    fn update_io(self, i: Input) -> (Self, Self::Output);
}

/// A type which can functionally update (ie: with move-semantics) given an `Input` to a new state
///
/// Every `UpdateInput<I>` type is an `UpdateIO<I, Output=()>` type by a blanket impl, making it more convenient to implement this trait for [UpdateIO]-consumer code.
pub trait UpdateInput<Input>: Sized {
    /// Given `i` update `self` to a new state [Self]
    fn update_input(self, i: Input) -> Self;
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
