use derive_more::{AsRef, Deref, DerefMut};
use derive_new::new;

use crate::into::{IntoNextWith, IntoUpdate};

/// Updates an inner state every [Self::interval] updates
#[derive(Debug, Deref, DerefMut, AsRef, new)]
pub struct Cycler<T> {
    #[deref]
    #[deref_mut]
    #[as_ref]
    inner: T,

    /// The number of updates before updating the inner state `T`
    pub interval: usize,

    /// The total number of updates
    #[new(default)]
    pub generation: usize,
}

impl<T, I> IntoNextWith<I> for Cycler<T>
where
    T: IntoUpdate<I>,
{
    type Next = Self;

    fn into_next_with(mut self, input: I) -> Self {
        self.generation += 1;
        if self.generation.is_multiple_of(self.interval) {
            self.inner = self.inner.into_next_with(input);
        }
        self
    }
}
