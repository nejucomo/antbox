use crate::into::IntoNextWith;

/// Any [IntoNextWith] with [Next](IntoNextWith::Next) as `Option<Self>` is an [IntoOptUpdate] for the same input `I`
pub trait IntoOptUpdate<I>: IntoNextWith<I, Next = Option<Self>> {
    /// A synonym for [IntoNextWith::into_next_with]
    fn into_opt_update(self, input: I) -> Option<Self> {
        self.into_next_with(input)
    }
}

impl<B, I> IntoOptUpdate<I> for B where B: IntoNextWith<I, Next = Option<Self>> {}
