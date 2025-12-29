use std::ops::Mul;

use derive_more::Into;

use crate::NFError;

/// A newtype wrapping [f32] on the interval `[0, 1]`
#[derive(Copy, Clone, Debug, Into)]
pub struct NF(f32);

impl NF {
    /// Wrap `f`, panicking if `f` is out-of-range
    pub fn new(f: f32) -> Self {
        Self::try_from(f).unwrap()
    }
}

impl TryFrom<f32> for NF {
    type Error = NFError;

    fn try_from(f: f32) -> Result<Self, Self::Error> {
        if (0f32..=1f32).contains(&f) {
            Ok(NF(f))
        } else {
            Err(NFError { f })
        }
    }
}

impl Mul for NF {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.0)
    }
}

impl Mul<NF> for f32 {
    type Output = f32;

    fn mul(self, rhs: NF) -> Self::Output {
        self * rhs.0
    }
}
