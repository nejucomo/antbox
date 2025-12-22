use crate::takeinto::TakeIntoNext;

/// A transitional state which leads to
pub trait IntoNext: Sized + TakeIntoNext<()> {
    /// Convert the current state into the next state
    fn into_next(self) -> Self::Next {
        self.into_next_with(())
    }
}

impl<B> IntoNext for B where B: Sized + TakeIntoNext<()> {}
