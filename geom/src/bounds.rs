use derive_more::{From, Into};
use derive_new::new;
use try_from_unwrap::TryFromUnwrap;

use crate::{BoundPoint, Point};

/// Two-dimensional area bounds
#[derive(Copy, Clone, Debug, From, Into, new)]
pub struct Bounds {
    /// The width
    pub width: u32,
    /// The height
    pub height: u32,
}

impl Bounds {
    /// The contained area
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Iterate over the [BoundPoint]s herein
    pub fn iter_points(&self) -> impl Iterator<Item = BoundPoint> {
        (0..usize::tfu(self.area()))
            .map(u32::tfu)
            .map(|ix| Point::new(ix % self.width, ix / self.width))
            .map(|pt| BoundPoint::new(pt, *self))
    }
}
