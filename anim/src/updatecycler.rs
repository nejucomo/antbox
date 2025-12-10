use derive_more::{Deref, DerefMut};
use derive_new::new;
use mealy_machine::IntoNext;

/// Updates an inner state every [Self::interval] updates
#[derive(Debug, Deref, DerefMut, new)]
pub struct UpdateCycler<T> {
    #[deref]
    #[deref_mut]
    inner: T,

    /// The number of updates before updating the inner state `T`
    pub interval: usize,

    /// The current number of `self` updates since the last inner update
    #[new(default)]
    pub current: usize,
}

impl<T> IntoNext for UpdateCycler<T>
where
    T: IntoNext,
{
    fn into_next(self) -> Self {
        let UpdateCycler {
            inner,
            interval,
            current,
        } = self;

        let current = if current >= interval { 0 } else { current + 1 };

        UpdateCycler {
            inner: if current == 0 {
                inner.into_next()
            } else {
                inner
            },
            interval,
            current,
        }
    }
}
