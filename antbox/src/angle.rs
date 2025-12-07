use std::f32::consts::TAU;
use std::ops::Add;

use derive_more::Into;

/// An angle in radians
#[derive(Copy, Clone, Into)]
pub struct Angle(f32);

impl Angle {
    pub fn cos(self) -> f32 {
        self.0.cos()
    }

    pub fn sin(self) -> f32 {
        self.0.sin()
    }
}

impl From<f32> for Angle {
    fn from(raw: f32) -> Self {
        Angle(raw.rem_euclid(TAU))
    }
}

impl<A: Into<Angle>> Add<A> for Angle {
    type Output = Angle;

    fn add(self, rhs: A) -> Self::Output {
        let other: Angle = rhs.into();
        Angle::from(self.0 + other.0)
    }
}

impl std::fmt::Debug for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "∡{:.3}𝛕", self.0 / TAU)
    }
}
