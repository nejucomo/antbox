use derive_more::{From, Into};
use derive_new::new;
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
use speedy2d::dimen::Vec2;

use crate::angle::Angle;

/// A two-dimensional vector using polar coordinates
#[derive(Copy, Clone, Debug, new, From, Into)]
pub struct TrigVec {
    /// The vector's [Angle]
    #[new(into)]
    pub angle: Angle,
    /// The vector's distance
    pub distance: f32,
}

impl TrigVec {
    /// The `x` cartesian coordinate
    pub fn x(self) -> f32 {
        self.angle.cos() * self.distance
    }

    /// The `y` cartesian coordinate
    pub fn y(self) -> f32 {
        self.angle.sin() * self.distance
    }

    /// Convert to a [Vec2]
    pub fn into_vec2(self) -> Vec2 {
        self.into()
    }

    /// Rotate the vector by `angle` in a counter-clockwise direction
    pub fn rotate<A: Into<Angle>>(self, angle: A) -> Self {
        TrigVec::new(self.angle + angle, self.distance)
    }

    /// Scale the vector by `factor`
    pub fn scale(self, factor: f32) -> Self {
        TrigVec::new(self.angle, self.distance * factor)
    }
}

impl From<TrigVec> for Vec2 {
    fn from(tv: TrigVec) -> Vec2 {
        Vec2::new(tv.x(), tv.y())
    }
}

impl Distribution<TrigVec> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TrigVec {
        TrigVec::new(rng.random::<Angle>(), rng.random_range(0f32..1f32))
    }
}
