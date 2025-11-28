use std::num::TryFromIntError;

use derive_more::{From, Into};
use derive_new::new;
use try_from_unwrap::TryFromUnwrap as _;

use crate::{Bounds, Point, Scalar};

/// A [Point] in the context
#[derive(Copy, Clone, Debug, From, Into, new)]
pub struct BoundPoint<T>
where
    T: Scalar,
    usize: TryFrom<T, Error = TryFromIntError>,
{
    pt: Point<T>,
    bounds: Bounds<T>,
}

impl<T> From<BoundPoint<T>> for usize
where
    T: Scalar,
    usize: TryFrom<T, Error = TryFromIntError>,
{
    fn from(bp: BoundPoint<T>) -> Self {
        usize::tfu(bp.pt.y * bp.bounds.width + bp.pt.x)
    }
}
