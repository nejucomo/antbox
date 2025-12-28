use std::ops::Add;

use derive_more::{From, Into};
use derive_new::new;

use crate::Polar;

/// A two-dimensional vector using cartesian coordinates
#[derive(Copy, Clone, Debug, new, From, Into)]
pub struct Point {
    /// The x coordinate
    pub x: f32,
    /// The y coordinate
    pub y: f32,
}

impl From<Polar> for Point {
    fn from(Polar { angle, distance }: Polar) -> Self {
        let (s, c) = angle.sin_cos();
        let d = f32::from(distance);
        Point { x: c * d, y: s * d }
    }
}

impl<P> Add<P> for Point
where
    P: Into<Point>,
{
    type Output = Self;

    fn add(self, rhs: P) -> Self::Output {
        let Point { x, y } = rhs.into();
        Point {
            x: self.x + x,
            y: self.y + y,
        }
    }
}
