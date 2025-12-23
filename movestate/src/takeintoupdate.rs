use crate::TakeIntoNext;

/// `(S, I) -> S`: Produce a new `Self` state from an input; aka a _Moore Machine_
///
/// Notice that this blanked extension of [TakeIntoNext] applies to _any_ `Next: Into<Self>` and is more general than just `Next = Self`.
pub trait TakeIntoUpdate<I>: Sized + TakeIntoNext<I, Next: Into<Self>> {
    /// Take `self` and `input` directly into the next `Self` value
    fn take_into_update(self, input: I) -> Self {
        self.take_into_next(input).into()
    }
}

impl<B, I> TakeIntoUpdate<I> for B where B: Sized + TakeIntoNext<I, Next: Into<B>> {}
