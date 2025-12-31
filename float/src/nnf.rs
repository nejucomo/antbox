use std::num::NonZero;
use std::ops::{Add, Div, Mul};

use derive_more::Into;

use crate::BoundedFloatError;

/// <u>N</u>on-<u>N</u>egative <u>F</u>inite: a newtype wrapping [f32] on the interval `[0, 1)`
#[derive(Copy, Clone, Debug, Into)]
pub struct NNF(f32);

impl NNF {
    /// Wrap `f`, panicking if `f` is out-of-range
    pub const fn fromp_f32(f: f32) -> Self {
        // Self::try_from_f32(f).unwrap()
        if let Ok(s) = Self::try_from_f32(f) {
            s
        } else {
            panic!("`NNF::fromp_f32(...)` failure");
        }
    }

    /// Try to construct from a raw [f32]
    pub const fn try_from_f32(f: f32) -> Result<Self, BoundedFloatError> {
        if 0.0 <= f && f.is_finite() {
            Ok(NNF(f))
        } else {
            Err(BoundedFloatError { f })
        }
    }
}

impl TryFrom<f32> for NNF {
    type Error = BoundedFloatError;

    fn try_from(f: f32) -> Result<Self, Self::Error> {
        Self::try_from_f32(f)
    }
}

impl Add for NNF {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Mul for NNF {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul<NNF> for f32 {
    type Output = f32;

    fn mul(self, rhs: NNF) -> Self::Output {
        self * rhs.0
    }
}

impl Div<NonZero<usize>> for NNF {
    type Output = NNF;

    fn div(self, rhs: NonZero<usize>) -> Self::Output {
        Self(self.0 / rhs.get() as f32)
    }
}
