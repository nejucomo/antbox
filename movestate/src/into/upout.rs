use crate::into::IntoNextWith;
use crate::next::Stout;

/// Any [IntoNextWith] whose [Next](IntoNextWith::Next) can convert into a `Stout<Self, O>`an [IntoUpdateWithOutput]: `(S, I) -> (S, O)`
///
/// This is also known as a _Mealy Machine_ which responds to an input with a new `Self` state and an output.
pub trait IntoUpdateWithOutput<I, O>: IntoNextWith<I, Next: Into<Stout<Self, O>>> {
    /// Convert into our next state and output
    fn into_update_output(self, input: I) -> (Self, O) {
        self.into_self_stout(input).into()
    }

    /// Convert into our [IntoNextWith::Next]'s associated [Stout]
    ///
    /// Direct consumer code may find [Self::into_update_output] more ergonomic
    fn into_self_stout(self, input: I) -> Stout<Self, O> {
        self.into_next_with(input).into()
    }
}

impl<B, I, O> IntoUpdateWithOutput<I, O> for B where B: IntoNextWith<I, Next: Into<Stout<Self, O>>> {}
