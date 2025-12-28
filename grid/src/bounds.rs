use std::str::FromStr;

use derive_more::{From, Into};
use derive_new::new;
use rand::Rng;
use rand::distr::Distribution;

use crate::{Coord, GridCoord};

/// Two-dimensional area bounds
#[derive(Copy, Clone, From, Into, new, Eq, Ord, PartialEq, PartialOrd)]
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
    pub fn bind<P: Into<Coord>>(self, pt: P) -> Option<GridCoord> {
        let pt = pt.into();
        if pt.x < self.width && pt.y < self.height {
            Some(GridCoord::new(pt, self))
        } else {
            None
        }
    }

    /// Iterate over the [GridCoord]s herein
    pub fn iter_points(self) -> impl Iterator<Item = GridCoord> {
        (0..self.area()).map(move |ix| self.ix_to_bp(ix))
    }

    pub(crate) fn ix_to_bp(self, ix: usize) -> GridCoord {
        assert!(ix < self.area());
        GridCoord::new(Coord::new(ix % self.width, ix / self.width), self)
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

impl std::fmt::Debug for Bounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Bounds { width, height } = self;
        write!(f, "[W{width} H{height}]")
    }
}

impl Distribution<GridCoord> for Bounds {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GridCoord {
        self.ix_to_bp(rng.random_range(0..self.area()))
    }
}
