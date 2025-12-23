use derive_more::{AsRef, Deref, DerefMut};
use derive_new::new;
use movestate::TakeIntoNext;

use crate::TickTimer;

/// Limits [TakeIntoNext] updates to an inner state by a [TickTimer]
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

impl<T, I> TakeIntoNext<I> for RateLimiter<T>
where
    T: TakeIntoNext<I, Next: Into<T>>,
{
    type Next = Self;

    fn take_into_next(mut self, input: I) -> Self {
        if self.tt.delta_update().is_late() {
            self.inner = self.inner.take_into_next(input).into();
        }
        self
    }
}
