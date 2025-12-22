/// A state which can incorporate an `I` into [Self::Next]
pub trait IntoNextWith<I>: Sized {
    /// The type after transforming with the input
    type Next;

    /// Transform `self` with `input` into a [Self::Next] state
    fn into_next_with(self, input: I) -> Self::Next;
}
