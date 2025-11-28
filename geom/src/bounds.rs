use std::num::TryFromIntError;

use derive_more::{From, Into};
use derive_new::new;
use try_from_unwrap::TryFromUnwrap;

use crate::{BoundPoint, Point, Scalar};

/// Two-dimensional area bounds
#[derive(Copy, Clone, Debug, From, Into, new)]
pub struct Bounds<T>
where
    T: Scalar,
    usize: TryFrom<T, Error = TryFromIntError>,
{
    /// The width
    pub width: T,
    /// The height
    pub height: T,
}

impl<T> Bounds<T>
where
    T: Scalar,
    usize: TryFrom<T, Error = TryFromIntError>,
{
    /// The contained area
    pub fn area(&self) -> T {
        self.width * self.height
    }

    /// Iterate over the [BoundPoint]s herein
    pub fn iter_points(&self) -> impl Iterator<Item = BoundPoint<T>> {
        (0..usize::tfu(self.area()))
            .map(T::tfu)
            .map(|ix| ix.div_mod_floor(&self.width))
            .map(Point::from)
            .map(|pt| BoundPoint::new(pt, *self))
    }
}
