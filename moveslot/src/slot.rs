use crate::MapInPlace;

/// A wrapper type enabling mutation via ownership-moving closures
//
// # Invariants
//
// `self.0.is_some()`
#[derive(Debug)]
pub struct MoveSlot<T>(Option<T>);

impl<T> MapInPlace<T> for MoveSlot<T> {
    fn unwrap_state(self) -> T {
        self.0.unwrap_state()
    }

    fn opt_state(self) -> Option<T> {
        self.0
    }

    fn mip_out<F, O>(&mut self, f: F) -> O
    where
        F: FnOnce(T) -> (T, O),
    {
        self.0.mip_out(f)
    }

    fn mip_out_opt<F, O>(&mut self, f: F) -> Option<O>
    where
        F: FnOnce(T) -> Option<(T, O)>,
    {
        self.0.mip_out_opt(f)
    }

    fn mip_out_res<F, O, E>(&mut self, f: F) -> Result<O, E>
    where
        F: FnOnce(T) -> Result<(T, O), E>,
    {
        self.0.mip_out_res(f)
    }
}

mod stdimpls {
    use std::ops::{Deref, DerefMut};

    use super::MoveSlot;

    impl<T> Default for MoveSlot<T>
    where
        T: Default,
    {
        fn default() -> Self {
            Self::from(T::default())
        }
    }

    impl<T> From<T> for MoveSlot<T> {
        fn from(v: T) -> Self {
            Self(Some(v))
        }
    }

    impl<T> Deref for MoveSlot<T> {
        type Target = T;

        fn deref(&self) -> &T {
            self.0.as_ref().unwrap()
        }
    }

    impl<T> DerefMut for MoveSlot<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.0.as_mut().unwrap()
        }
    }

    impl<T, U> AsRef<U> for MoveSlot<T>
    where
        T: AsRef<U>,
    {
        fn as_ref(&self) -> &U {
            self.deref().as_ref()
        }
    }

    impl<T, U> AsMut<U> for MoveSlot<T>
    where
        T: AsMut<U>,
    {
        fn as_mut(&mut self) -> &mut U {
            self.deref_mut().as_mut()
        }
    }
}
