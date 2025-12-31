use std::ops::Add;

use antbox_float::{NNF, Norm};
use derive_more::{From, Into};
use derive_new::new;
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};

use crate::angle::Angle;
use crate::{Distance, Point, Transformable};

/// A two-dimensional vector using polar coordinates
#[derive(Copy, Clone, Debug, new, From, Into)]
pub struct Polar {
    /// The vector's [Angle]
    #[new(into)]
    pub angle: Angle,
    /// The vector's distance
    #[new(into)]
    pub distance: Distance,
}

impl From<Point> for Polar {
    fn from(Point { x, y }: Point) -> Self {
        Polar {
            angle: y.atan2(x).into(),
            distance: (x.powi(2) + y.powi(2)).sqrt().try_into().unwrap(),
        }
    }
}

impl<P> Add<P> for Polar
where
    P: Into<Point>,
{
    type Output = Self;

    fn add(self, rhs: P) -> Self::Output {
        // TODO: Is there a more efficient way to do this?
        Self::from(Point::from(self) + rhs)
    }
}

impl Transformable for Polar {
    fn rotate_by_angle(self, a: Angle) -> Self {
        Polar {
            angle: self.angle + a,
            ..self
        }
    }

    fn scale_by_nnf(self, s: NNF) -> Self {
        Polar {
            distance: self.distance * s,
            ..self
        }
    }

    fn translate_by_point(self, p: Point) -> Self {
        self + p
    }
}

impl Distribution<Polar> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Polar {
        rng.random::<Angle>().with_distance(rng.random::<Norm>())
    }
}
