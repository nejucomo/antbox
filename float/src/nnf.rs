use std::num::NonZero;
use std::ops::{Add, Div, Mul};

use derive_more::Into;

use crate::{BoundedFloatError, Norm, PowUnsigned};

/// <u>N</u>on-<u>N</u>egative <u>F</u>inite: a newtype wrapping [f32] on the interval `[0, ∞)`
#[derive(Copy, Clone, Debug, Into, PartialEq, PartialOrd)]
pub struct NNF(f32);

impl NNF {
    #[allow(missing_docs)]
    pub const ZERO: NNF = NNF(0f32);
    #[allow(missing_docs)]
    pub const ONE: NNF = NNF(1f32);

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

impl From<Norm> for NNF {
    fn from(n: Norm) -> Self {
        Self(n.into())
    }
}

impl TryFrom<f32> for NNF {
    type Error = BoundedFloatError;

    fn try_from(f: f32) -> Result<Self, Self::Error> {
        Self::try_from_f32(f)
    }
}

impl Eq for NNF {}

// We allow this suspicious implementation because `self.0` is never `nan` or `±∞`
#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for NNF {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
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

impl Mul<Norm> for NNF {
    type Output = NNF;

    fn mul(self, rhs: Norm) -> Self::Output {
        self * NNF::from(rhs)
    }
}

impl Mul<f32> for NNF {
    type Output = f32;

    fn mul(self, rhs: f32) -> Self::Output {
        self.0 * rhs
    }
}

impl Mul<usize> for NNF {
    type Output = NNF;

    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * rhs as f32)
    }
}

impl PowUnsigned for NNF {
    fn pow_nnf(self, pow: NNF) -> Self {
        Self(self.0.powf(pow.0))
    }

    fn pow_u32(self, pow: u32) -> Self {
        Self(self.0.powi(pow.try_into().unwrap()))
    }
}

impl Div<NonZero<usize>> for NNF {
    type Output = NNF;

    fn div(self, rhs: NonZero<usize>) -> Self::Output {
        Self(self.0 / rhs.get() as f32)
    }
}
