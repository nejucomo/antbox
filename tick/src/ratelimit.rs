use derive_more::{AsRef, Deref, DerefMut};
use derive_new::new;
use movestate::{Transform, Update};

use crate::TickTimer;

/// Limits [Transform] updates to an inner state by a [TickTimer]
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

impl<T, I> Transform<I> for RateLimiter<T>
where
    T: Update<I>,
{
    type Next = Self;

    fn transform(mut self, input: I) -> Self {
        if self.tt.delta_update().is_late() {
            self.inner = self.inner.transform(input);
        }
        self
    }
}
