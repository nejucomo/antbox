use std::ops::Mul;

use derive_more::Into;
use rand::distr::{Distribution, StandardUniform};

use crate::BoundedFloatError;

/// A newtype wrapping [f32] on the interval `[0, 1]`
#[derive(Copy, Clone, Debug, Into)]
pub struct Norm(f32);

impl Norm {
    #[allow(missing_docs)]
    pub const ZERO: Norm = Norm(0f32);
    #[allow(missing_docs)]
    pub const ONE: Norm = Norm(1f32);
    #[allow(missing_docs)]
    pub const HALF: Norm = Norm(0.5f32);

    /// Wrap `f`, panicking if `f` is out-of-range
    pub const fn fromp_f32(f: f32) -> Self {
        // Self::try_from_f32(f).unwrap()
        if let Ok(s) = Self::try_from_f32(f) {
            s
        } else {
            panic!("`Norm::fromp_f32(...)` failure");
        }
    }

    /// Try to construct from a raw [f32]
    pub const fn try_from_f32(f: f32) -> Result<Self, BoundedFloatError> {
        if 0.0 <= f && f <= 1.0 {
            Ok(Norm(f))
        } else {
            Err(BoundedFloatError { f })
        }
    }

    /// Convert to a norm float approximating `u /` [`u8::MAX`]
    pub const fn from_u8(u: u8) -> Self {
        Self(u as f32 / u8::MAX as f32)
    }

    /// Convert to a norm float approximating `u /` [`u8::MAX`]
    pub const fn interpolate(self, other: Self, proportion: Self) -> Self {
        Self((other.0 - self.0) * proportion.0 + self.0)
    }
}

impl TryFrom<f32> for Norm {
    type Error = BoundedFloatError;

    fn try_from(f: f32) -> Result<Self, Self::Error> {
        Self::try_from_f32(f)
    }
}

impl Mul for Norm {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul<Norm> for f32 {
    type Output = f32;

    fn mul(self, rhs: Norm) -> Self::Output {
        self * rhs.0
    }
}

impl Distribution<Norm> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Norm {
        Norm(rng.random_range(0f32..=1f32))
    }
}
