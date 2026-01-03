use crate::TakeIntoNext;

/// `S -> N`
pub trait IntoNext: Sized + TakeIntoNext<()> {
    /// `S -> N`
    ///
    /// Transition `self` into [Next](TakeIntoNext::Next)
    fn into_next(self) -> Self::Next {
        self.take_into_next(())
    }
}

impl<B> IntoNext for B where B: TakeIntoNext<()> {}
