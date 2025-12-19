/// A state which can incorporate an `I` into [Self::Next]
pub trait Transform<I>: Sized {
    /// The type after updating with the input
    type Next;

    /// transform `self` with `input` into a [Self::Next] state
    fn transform(self, input: I) -> Self::Next;
}
