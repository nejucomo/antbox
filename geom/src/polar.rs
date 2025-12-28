use std::f32::consts::PI;
use std::ops::Add;

use derive_more::{From, Into};
use derive_new::new;
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
use speedy2d::dimen::Vec2;

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
            distance: (x.powi(2) + y.powi(2)).sqrt().into(),
        }
    }
}

impl From<Polar> for Vec2 {
    fn from(p: Polar) -> Self {
        let Point { x, y } = p.into();
        Vec2 { x, y }
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
    fn rotate<A>(self, a: A) -> Self
    where
        A: Into<Angle>,
    {
        Polar {
            angle: self.angle + a.into(),
            ..self
        }
    }

    fn scale(self, s: f32) -> Self {
        Polar {
            distance: self.distance * s.abs(),
            angle: self.angle + if s >= 0.0 { 0.0 } else { PI },
        }
    }

    fn translate(self, delta: Point) -> Self {
        self + delta
    }
}

impl Distribution<Polar> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Polar {
        rng.random::<Angle>()
            .with_distance(rng.random_range(0f32..1f32))
    }
}
