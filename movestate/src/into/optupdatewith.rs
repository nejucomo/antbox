use crate::into::IntoNextWith;

/// Any [IntoNextWith] with [Next](IntoNextWith::Next) as `Option<Self>`
pub trait IntoOptUpdateWith<I>: IntoNextWith<I, Next = Option<Self>> {
    /// A synonym for [IntoNextWith::into_next_with]
    fn into_opt_update(self, input: I) -> Option<Self> {
        self.into_next_with(input)
    }
}

impl<B, I> IntoOptUpdateWith<I> for B where B: IntoNextWith<I, Next = Option<Self>> {}
