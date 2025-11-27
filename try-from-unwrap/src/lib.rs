//! A trait providing shorthand for `T::try_from(v).unwrap()`
#![deny(unsafe_code, missing_docs)]

use std::fmt::Debug;

/// A trait providing shorthand for `T::try_from(v).unwrap()` as `T::tfu(v)`
pub trait TryFromUnwrap<T>: TryFrom<T>
where
    Self::Error: Debug,
{
    /// Equivalent to `Self::try_from(v).unwrap()`
    fn tfu(v: T) -> Self {
        Self::try_from(v).unwrap()
    }
}

impl<F, T> TryFromUnwrap<T> for F
where
    F: TryFrom<T>,
    F::Error: Debug,
{
}
