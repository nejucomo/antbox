use derive_more::{From, Into};
use derive_new::new;

use crate::{BoundPoint, Point};

/// Two-dimensional area bounds
#[derive(Copy, Clone, Debug, From, Into, new, PartialEq)]
pub struct Bounds {
    /// The width
    pub width: usize,
    /// The height
    pub height: usize,
}

impl Bounds {
    /// The contained area
    pub fn area(&self) -> usize {
        self.width * self.height
    }

    /// Iterate over the [BoundPoint]s herein
    pub fn iter_points(self) -> impl Iterator<Item = BoundPoint> {
        (0..self.area()).map(move |ix| self.ix_to_bp(ix))
    }

    pub(crate) fn ix_to_bp(self, ix: usize) -> BoundPoint {
        assert!(ix < self.area());
        BoundPoint::new(Point::new(ix % self.width, ix / self.width), self)
    }
}

impl From<(u32, u32)> for Bounds {
    fn from((w, h): (u32, u32)) -> Self {
        Bounds::new(usize::try_from(w).unwrap(), usize::try_from(h).unwrap())
    }
}
