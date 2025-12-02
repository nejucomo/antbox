use derive_more::{From, Into};
use derive_new::new;
use try_from_unwrap::TryFromUnwrap;

use crate::{BoundPoint, Point};

/// Two-dimensional area bounds
#[derive(Copy, Clone, Debug, From, Into, new, PartialEq)]
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
    pub fn iter_points(self) -> impl Iterator<Item = BoundPoint> {
        (0..usize::tfu(self.area())).map(move |ix| self.ix_to_bp(ix))
    }

    pub(crate) fn ix_to_bp(self, ix: usize) -> BoundPoint {
        let u = u32::tfu(ix);
        assert!(u < self.area());
        BoundPoint::new(Point::new(u % self.width, u / self.width), self)
    }
}
