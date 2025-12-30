use std::ops::Mul;

use derive_more::Into;

use crate::BoundedFloatError;

/// A newtype wrapping [f32] on the interval `[0, 1]`
#[derive(Copy, Clone, Debug, Into)]
pub struct NormF32(f32);

impl NormF32 {
    /// Wrap `f`, panicking if `f` is out-of-range
    pub const fn from_f32(f: f32) -> Self {
        // Self::try_from_f32(f).unwrap()
        if let Ok(s) = Self::try_from_f32(f) {
            s
        } else {
            panic!("`NormF32::from_f32(...)` failure");
        }
    }

    /// Try to construct from a raw [f32]
    pub const fn try_from_f32(f: f32) -> Result<Self, BoundedFloatError> {
        if 0.0 <= f && f <= 1.0 {
            Ok(NormF32(f))
        } else {
            Err(BoundedFloatError { f })
        }
    }

    /// Convert to a norm float approximating `u /` [`u8::MAX`]
    pub const fn from_u8(u: u8) -> Self {
        Self::from_f32(u as f32 / u8::MAX as f32)
    }
}

impl TryFrom<f32> for NormF32 {
    type Error = BoundedFloatError;

    fn try_from(f: f32) -> Result<Self, Self::Error> {
        Self::try_from_f32(f)
    }
}

impl Mul for NormF32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_f32(self.0 * rhs.0)
    }
}

impl Mul<NormF32> for f32 {
    type Output = f32;

    fn mul(self, rhs: NormF32) -> Self::Output {
        self * rhs.0
    }
}
