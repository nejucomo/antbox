use std::ops::{Deref, DerefMut};

use crate::mutable::Update;
use crate::mutable::optext::OptionExt as _;
use crate::take_into::TakeIntoStarg;

/// Hold a state and contain functional transitions within a mutable interface
//
// # Invariant:
//
// The only time `self.0` is `None` is within an incomplete transition
#[derive(Debug)]
pub struct Slot<T>(Option<T>);

impl<T> Slot<T> {
    /// Unwrap the inner state
    pub fn into_inner(self) -> T {
        self.0.unslot()
    }
}

impl<T, I, O> Update<I, O> for Slot<T>
where
    T: TakeIntoStarg<I, O>,
{
    fn update(&mut self, input: I) -> O {
        self.0.mealy_map(|s| s.take_into_starg(input).into())
    }
}

impl<T> Default for Slot<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::from(T::default())
    }
}

impl<T> From<T> for Slot<T> {
    fn from(v: T) -> Self {
        Self(Some(v))
    }
}

impl<T> Deref for Slot<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.0.as_ref().unslot()
    }
}

impl<T> DerefMut for Slot<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unslot()
    }
}

impl<T, U> AsRef<U> for Slot<T>
where
    T: AsRef<U>,
{
    fn as_ref(&self) -> &U {
        self.deref().as_ref()
    }
}

impl<T, U> AsMut<U> for Slot<T>
where
    T: AsMut<U>,
{
    fn as_mut(&mut self) -> &mut U {
        self.deref_mut().as_mut()
    }
}
