use crate::TakeIntoNext;

/// `S -> N`: Transition `self` into a [Next](TakeIntoNext::Next) value
pub trait IntoNext: Sized + TakeIntoNext<()> {
    /// Transition `self` into [Next](TakeIntoNext::Next)
    fn into_next(self) -> Self::Next {
        self.take_into_next(())
    }
}

impl<B> IntoNext for B where B: TakeIntoNext<()> {}
