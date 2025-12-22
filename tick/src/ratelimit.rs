use derive_more::{AsRef, Deref, DerefMut};
use derive_new::new;
use movestate::into::{IntoNextWith, IntoUpdateWithOutput};

use crate::TickTimer;

/// Limits [IntoNextWith] updates to an inner state by a [TickTimer]
///
/// Note: all inputs after the first within a given interval are simply dropped.
#[derive(Debug, Deref, DerefMut, AsRef, new)]
pub struct RateLimiter<T> {
    #[deref]
    #[deref_mut]
    #[as_ref]
    inner: T,

    tt: TickTimer,
}

impl<T, I> IntoNextWith<I> for RateLimiter<T>
where
    T: IntoUpdateWithOutput<I>,
{
    type Next = Self;

    fn into_next_with(mut self, input: I) -> Self {
        if self.tt.delta_update().is_late() {
            self.inner = self.inner.into_next_with(input);
        }
        self
    }
}
