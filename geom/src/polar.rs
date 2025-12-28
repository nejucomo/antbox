use derive_more::{From, Into};
use derive_new::new;
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
use speedy2d::dimen::Vec2;

use crate::Distance;
use crate::angle::Angle;

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

impl Polar {
    /// The shortest [Distance] from the Y-axis
    pub fn x(self) -> Distance {
        self.distance * self.angle.cos()
    }

    /// The shortest [Distance] from the X-axis
    pub fn y(self) -> Distance {
        self.distance * self.angle.sin()
    }

    /// Convert to a [Vec2]
    pub fn into_vec2(self) -> Vec2 {
        self.into()
    }

    /// Rotate the vector by `angle` in a counter-clockwise direction
    pub fn rotate<A: Into<Angle>>(self, angle: A) -> Self {
        (self.angle + angle).with_distance(self.distance)
    }

    /// Scale the vector by `factor`
    pub fn scale(self, factor: f32) -> Self {
        self.angle.with_distance(self.distance * factor)
    }
}

impl From<Polar> for Vec2 {
    fn from(tv: Polar) -> Vec2 {
        Vec2::new(tv.x().into(), tv.y().into())
    }
}

impl Distribution<Polar> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Polar {
        rng.random::<Angle>()
            .with_distance(rng.random_range(0f32..1f32))
    }
}
