use std::str::FromStr;

use derive_more::{From, Into};
use derive_new::new;
use rand::Rng;
use rand::distr::Distribution;

use crate::{BoundPoint, Point};

/// Two-dimensional area bounds
#[derive(Copy, Clone, Debug, From, Into, new, Eq, Ord, PartialEq, PartialOrd)]
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

    /// Bind a point, if it's within our bounds
    pub fn bind<P: Into<Point>>(self, pt: P) -> Option<BoundPoint> {
        let pt = pt.into();
        if pt.x < self.width && pt.y < self.height {
            Some(BoundPoint::new(pt, self))
        } else {
            None
        }
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

impl FromStr for Bounds {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (w, h) = s.split_once('x').ok_or("missing 'x' infix")?;
        let w = w.parse().map_err(|_| "parse error in width")?;
        let h = h.parse().map_err(|_| "parse error in height")?;
        Ok(Bounds::new(w, h))
    }
}

impl Distribution<BoundPoint> for Bounds {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BoundPoint {
        self.ix_to_bp(rng.random_range(0..self.area()))
    }
}
