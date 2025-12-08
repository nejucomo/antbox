use std::ops::{Deref, DerefMut};

use crate::IOTransform;
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

    /// Transform the state
    pub fn transform<Input>(&mut self, input: Input) -> T::Output
    where
        T: IOTransform<Input>,
    {
        let prevstate = self.0.take().unslot();
        let (nextstate, output) = prevstate.transform_io(input);
        self.0 = Some(nextstate);
        output
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
