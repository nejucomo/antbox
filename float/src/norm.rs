use std::ops::Mul;

use derive_more::Into;

use crate::BoundedFloatError;

/// A newtype wrapping [f32] on the interval `[0, 1]`
#[derive(Copy, Clone, Debug, Into)]
pub struct NormF32(f32);

impl NormF32 {
    /// Wrap `f`, panicking if `f` is out-of-range
    pub fn new(f: f32) -> Self {
        Self::try_from(f).unwrap()
    }
}

impl TryFrom<f32> for NormF32 {
    type Error = BoundedFloatError;

    fn try_from(f: f32) -> Result<Self, Self::Error> {
        if (0f32..=1f32).contains(&f) {
            Ok(NormF32(f))
        } else {
            Err(BoundedFloatError { f })
        }
    }
}

impl Mul for NormF32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.0)
    }
}

impl Mul<NormF32> for f32 {
    type Output = f32;

    fn mul(self, rhs: NormF32) -> Self::Output {
        self * rhs.0
    }
}
