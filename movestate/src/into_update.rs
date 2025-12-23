use crate::{IntoNext, TakeIntoUpdate};

/// `S -> S`: Transition `self` into a next `Self` value, aka an _endomorphism_
pub trait IntoUpdate: IntoNext + TakeIntoUpdate<()> {
    /// Transition `self` into a next `Self` value
    fn into_update(self) -> Self {
        self.into_next().into()
    }
}

impl<B> IntoUpdate for B where B: IntoNext + TakeIntoUpdate<()> {}
