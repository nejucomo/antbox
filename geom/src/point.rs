use std::num::TryFromIntError;

use derive_more::{From, Into};
use derive_new::new;

use crate::Scalar;

/// A 2D point
#[derive(Copy, Clone, Debug, From, Into, new)]
pub struct Point<T>
where
    T: Scalar,
    usize: TryFrom<T, Error = TryFromIntError>,
{
    /// The x coordinate
    pub x: T,
    /// The y coordinate
    pub y: T,
}
