use std::f32::consts::TAU;
use std::ops::Add;

use derive_more::Into;
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};

use crate::{Distance, Polar};

/// An angle in radians
///
/// # Geomertry
///
/// An absolute/contextless [Angle] is from the positive X-axis measuring counterclockwise.
#[derive(Copy, Clone, Into)]
pub struct Angle(f32);

impl Angle {
    /// Produce a [Polar] from this [Angle] and a [Distance]
    pub fn with_distance<D>(self, distance: D) -> Polar
    where
        D: Into<Distance>,
    {
        Polar::new(self, distance)
    }

    /// The (sin, cos) of this angle
    pub fn sin_cos(self) -> (f32, f32) {
        self.0.sin_cos()
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
