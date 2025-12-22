use crate::into::IntoNextWith;

/// Any [IntoNextWith] with [Next](IntoNextWith::Next) as `Self` is an [IntoUpdate] for the same input `I`
pub trait IntoUpdate<I>: IntoNextWith<I, Next = Self> {
    /// A synonym for [IntoNextWith::into_next_with]
    fn into_update(self, input: I) -> Self {
        self.into_next_with(input)
    }
}

impl<B, I> IntoUpdate<I> for B where B: IntoNextWith<I, Next = Self> {}
