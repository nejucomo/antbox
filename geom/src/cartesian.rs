use std::ops::Add;

use derive_more::{From, Into};
use derive_new::new;

use crate::{Angle, Distance, Point, Polar};

/// A two-dimensional vector using cartesian coordinates
#[derive(Copy, Clone, Debug, new, From, Into)]
pub struct Cartesian {
    /// The x coordinate
    pub x: f32,
    /// The y coordinate
    pub y: f32,
}

impl Point for Cartesian {
    fn x(self) -> f32 {
        self.x
    }

    fn y(self) -> f32 {
        self.y
    }

    fn angle(self) -> Angle {
        Angle::from(self.y.atan2(self.x))
    }

    fn distance(self) -> Distance {
        Distance::from((self.x.powi(2) + self.y.powi(2)).sqrt())
    }
}

impl From<Polar> for Cartesian {
    fn from(p: Polar) -> Self {
        Cartesian { x: p.x(), y: p.y() }
    }
}

impl<P> Add<P> for Cartesian
where
    P: Point,
{
    type Output = Self;

    fn add(mut self, rhs: P) -> Self::Output {
        self.x += rhs.x();
        self.y += rhs.y();
        self
    }
}
