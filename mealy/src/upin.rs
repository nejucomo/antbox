use crate::IntoNext;

/// A type which can functionally update (ie: with move-semantics) given an `Input` to a new state
///
/// Every `UpdateInput<I>` type is an `UpdateIO<I, Output=()>` type by a blanket impl, making it more convenient to implement this trait for [UpdateIO](crate::UpdateIO)-consumer code.
pub trait UpdateInput<Input>: Sized {
    /// Given `i` update `self` to a new state [Self]
    fn update_input(self, i: Input) -> Self;
}

impl<T> UpdateInput<()> for T
where
    T: IntoNext,
{
    fn update_input(self, _: ()) -> Self {
        self.into_next()
    }
}
