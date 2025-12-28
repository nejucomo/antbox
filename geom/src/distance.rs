use std::ops::Mul;

use derive_more::Into;

/// A non-negative scalar distance, represented as [f32]
#[derive(Copy, Clone, Debug, Into, PartialEq, PartialOrd)]
pub struct Distance(f32);

impl From<f32> for Distance {
    fn from(f: f32) -> Self {
        assert!(!f.is_nan());
        assert!(f >= 0.0);
        Distance(f)
    }
}

impl Mul<f32> for Distance {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::from(self.0 * rhs)
    }
}
