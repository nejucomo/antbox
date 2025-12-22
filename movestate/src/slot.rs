use std::ops::{Deref, DerefMut};

use crate::into::IntoUpdate;
use crate::optext::OptionExt as _;

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

    /// Update the inner state
    pub fn update<I>(&mut self, input: I)
    where
        T: IntoUpdate<I>,
    {
        self.map(|t| t.into_update(input))
    }

    /// Map the inner state to a new value
    pub fn map<F>(&mut self, f: F)
    where
        F: FnOnce(T) -> T,
    {
        let next = f(self.0.take().unslot());
        self.0 = Some(next);
    }

    /// Map the inner state to a new value and output
    pub fn map_out<F, O>(&mut self, f: F) -> O
    where
        F: FnOnce(T) -> (T, O),
    {
        let (next, out) = f(self.0.take().unslot());
        self.0 = Some(next);
        out
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
