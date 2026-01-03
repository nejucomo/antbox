use derive_more::{AsRef, Deref, DerefMut};
use derive_new::new;
use mstate::MStateIn;

use crate::TickTimer;

/// Limits [MStateIn] updates to an inner state by a [TickTimer]
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

impl<T, I> MStateIn<I> for RateLimiter<T>
where
    T: MStateIn<I, Next: Into<T>>,
{
    type Next = Self;

    fn into_with(mut self, input: I) -> Self {
        if self.tt.delta_update().is_late() {
            self.inner = self.inner.into_with(input).into();
        }
        self
    }
}
