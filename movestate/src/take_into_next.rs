use crate::Starg;

/// `(S, I) -> N`: The base implementation trait which defines the [movestate](crate) family.
pub trait TakeIntoNext<I> {
    /// The next type produced when processing an `input`
    type Next;

    /// Take `self` and an `input` into a [Self::Next] value
    fn take_into_next(self, input: I) -> Self::Next;
}

impl<S, I> TakeIntoNext<()> for Starg<S, I>
where
    S: TakeIntoNext<I, Next = S>,
{
    type Next = S;

    fn take_into_next(self, (): ()) -> Self::Next {
        self.state.take_into_next(self.arg)
    }
}
