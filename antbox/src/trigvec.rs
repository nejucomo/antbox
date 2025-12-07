use derive_more::{From, Into};
use derive_new::new;
use speedy2d::dimen::Vec2;

use crate::angle::Angle;
// use speedy2d::dimen::Vec2;

#[derive(Copy, Clone, Debug, new, From, Into)]
pub struct TrigVec {
    #[new(into)]
    pub angle: Angle,
    pub distance: f32,
}

impl TrigVec {
    pub fn rotate<A: Into<Angle>>(self, angle: A) -> Self {
        TrigVec::new(self.angle + angle, self.distance)
    }

    pub fn into_vec2(self) -> Vec2 {
        self.into()
    }

    pub fn x(self) -> f32 {
        self.angle.cos() * self.distance
    }

    pub fn y(self) -> f32 {
        self.angle.sin() * self.distance
    }
}

impl From<TrigVec> for Vec2 {
    fn from(tv: TrigVec) -> Vec2 {
        Vec2::new(tv.x(), tv.y())
    }
}
