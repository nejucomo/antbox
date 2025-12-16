use derive_more::{AsRef, Deref, DerefMut};
use derive_new::new;

use crate::IntoNext;

/// Updates an inner state every [Self::interval] updates
#[derive(Debug, Deref, DerefMut, AsRef, new)]
pub struct Cycler<T> {
    #[deref]
    #[deref_mut]
    #[as_ref]
    inner: T,

    /// The number of updates before updating the inner state `T`
    pub interval: usize,

    /// The current number of `self` updates since the last inner update
    #[new(default)]
    pub current: usize,
}

impl<T> IntoNext for Cycler<T>
where
    T: IntoNext,
{
    fn into_next(self) -> Self {
        let Cycler {
            inner,
            interval,
            current,
        } = self;

        let current = if current >= interval { 0 } else { current + 1 };

        Cycler {
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
