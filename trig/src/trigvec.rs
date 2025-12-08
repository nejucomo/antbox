use derive_more::{From, Into};
use derive_new::new;
use speedy2d::dimen::Vec2;

use crate::angle::Angle;
// use speedy2d::dimen::Vec2;

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
    /// Rotate the vector by `angle` in a counter-clockwise direction
    pub fn rotate<A: Into<Angle>>(self, angle: A) -> Self {
        TrigVec::new(self.angle + angle, self.distance)
    }

    /// Convert to a [speedy2d::Vec2]
    pub fn into_vec2(self) -> Vec2 {
        self.into()
    }

    /// The `x` cartesian coordinate
    pub fn x(self) -> f32 {
        self.angle.cos() * self.distance
    }

    /// The `y` cartesian coordinate
    pub fn y(self) -> f32 {
        self.angle.sin() * self.distance
    }
}

impl From<TrigVec> for Vec2 {
    fn from(tv: TrigVec) -> Vec2 {
        Vec2::new(tv.x(), tv.y())
    }
}
