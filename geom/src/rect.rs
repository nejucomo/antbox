use std::num::TryFromIntError;

use derive_more::{From, Into};
use derive_new::new;

use crate::{Bounds, Point, Scalar};

/// A rectangle
#[derive(Copy, Clone, Debug, From, Into, new)]
pub struct Rect<T>
where
    T: Scalar,
    usize: TryFrom<T, Error = TryFromIntError>,
{
    top_left: Point<T>,
    bounds: Bounds<T>,
}
