use crate::Transform;

/// A transitional state which leads to
pub trait IntoNext: Sized + Transform<()> {
    /// Convert the current state into the next state
    fn into_next(self) -> Self::Next {
        self.transform(())
    }
}

impl<B> IntoNext for B where B: Sized + Transform<()> {}
