use std::ops::Add;

use derive_more::{From, Into};
use derive_new::new;
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};

use crate::angle::Angle;
use crate::{Cartesian, Distance, Point};

/// A two-dimensional vector using polar coordinates
#[derive(Copy, Clone, Debug, new, From, Into)]
pub struct Polar {
    /// The vector's [Angle]
    #[new(into)]
    angle: Angle,
    /// The vector's distance
    #[new(into)]
    distance: Distance,
}

impl Point for Polar {
    fn x(self) -> f32 {
        f32::from(self.distance) * self.angle.cos()
    }

    /// The shortest [f32] from the X-axis
    fn y(self) -> f32 {
        f32::from(self.distance) * self.angle.sin()
    }

    fn angle(self) -> Angle {
        self.angle
    }

    fn distance(self) -> Distance {
        self.distance
    }
}

impl From<Cartesian> for Polar {
    fn from(c: Cartesian) -> Self {
        Polar {
            angle: c.angle(),
            distance: c.distance(),
        }
    }
}

impl<P> Add<P> for Polar
where
    P: Point,
{
    type Output = Self;

    fn add(self, rhs: P) -> Self::Output {
        // TODO: Is there a more efficient way to do this?
        Self::from(Cartesian::from(self) + rhs)
    }
}

impl Distribution<Polar> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Polar {
        rng.random::<Angle>()
            .with_distance(rng.random_range(0f32..1f32))
    }
}
