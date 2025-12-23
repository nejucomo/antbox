/// `(S, I) -> N`: The base implementation trait which defines the [movestate](crate) family.
pub trait TakeIntoNext<I> {
    /// The next type produced when processing an `input`
    type Next;

    /// Take `self` and an `input` into a [Self::Next] value
    fn take_into_next(self, input: I) -> Self::Next;
}

// impl<S> TakeIntoNext<()> for S
// where
//     S: Iterator,
// {
//     type Next = Option<(S, S::Item)>;

//     fn take_into_next(mut self, (): ()) -> Self::Next {
//         self.next().map(|x| (self, x))
//     }
// }
