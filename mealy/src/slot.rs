use std::ops::{Deref, DerefMut};

use crate::optext::OptionExt as _;
use crate::{IntoNext, UpdateIO};

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

    /// Transform the state
    pub fn update_io<Input>(&mut self, input: Input) -> T::Output
    where
        T: UpdateIO<Input>,
    {
        let prevstate = self.0.take().unslot();
        let (nextstate, output) = prevstate.update_io(input);
        self.0 = Some(nextstate);
        output
    }

    /// Transform the state
    pub fn update_next(&mut self)
    where
        T: IntoNext,
    {
        self.update_io(());
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
