use std::f32::consts::TAU;
use std::ops::Add;

use derive_more::Into;
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};

/// An angle in radians
#[derive(Copy, Clone, Into)]
pub struct Angle(f32);

impl Angle {
    /// The cosine of this angle
    pub fn cos(self) -> f32 {
        self.0.cos()
    }

    /// The sine of this angle
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

impl Distribution<Angle> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Angle {
        Angle(rng.random_range(0f32..TAU))
    }
}
