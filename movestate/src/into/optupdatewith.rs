use crate::takeinto::TakeIntoNext;

/// Any [TakeIntoNext] with [Next](TakeIntoNext::Next) as `Option<Self>`
pub trait IntoOptUpdateWith<I>: TakeIntoNext<I, Next = Option<Self>> {
    /// A synonym for [TakeIntoNext::into_next_with]
    fn into_opt_update(self, input: I) -> Option<Self> {
        self.into_next_with(input)
    }
}

impl<B, I> IntoOptUpdateWith<I> for B where B: TakeIntoNext<I, Next = Option<Self>> {}
