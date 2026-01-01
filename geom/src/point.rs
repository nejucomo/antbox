use std::ops::{Add, Neg, Sub};

use antbox_float::NNF;
use derive_more::{From, Into};
use derive_new::new;

use crate::{Angle, Circle, Distance, Polar, Transformable, Vector};

/// A two-dimensional vector using cartesian coordinates
///
/// # Geometry
///  
/// X-axis is horizontal from left to right.
/// Y-axis is vertical from top to bottom.
#[derive(Copy, Clone, Debug, new, From, Into)]
pub struct Point {
    /// The x coordinate
    pub x: f32,
    /// The y coordinate
    pub y: f32,
}

impl Point {
    /// The origin point
    pub const ORIGIN: Point = Point { x: 0f32, y: 0f32 };

    /// The [Vector] which [start](Vector::start)s on `self` proceeding to [delta](Vector::delta)
    pub fn with_delta<P>(self, delta: P) -> Vector
    where
        P: Into<Point>,
    {
        Vector::new(self, delta)
    }

    /// The [Vector] which [start](Vector::start)s on `self` proceeding to the absolute point `to` (e.g. [delta](Vector::delta) `= to - start`)
    pub fn vector_to<P>(self, to: P) -> Vector
    where
        P: Into<Point>,
    {
        Vector::new(self, to.into() - self)
    }

    /// The [Circle] centered on `self` with `radius`
    pub fn with_radius<R>(self, radius: R) -> Circle
    where
        R: Into<Distance>,
    {
        Circle::new(self, radius)
    }

    /// The [Distance] from the [Point::ORIGIN]
    pub fn distance_from_origin(self) -> Distance {
        let Point { x, y } = self;

        Distance::fromp_f32((x.powi(2) + y.powi(2)).sqrt())
    }
}

impl Transformable for Point {
    fn rotate_by_angle(self, a: Angle) -> Self {
        Polar::from(self).rotate_by_angle(a).into()
    }

    fn scale_by_nnf(self, s: NNF) -> Self {
        Polar::from(self).scale_by_nnf(s).into()
    }

    fn translate_by_point(self, p: Point) -> Self {
        self + p
    }
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

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<P> Sub<P> for Point
where
    P: Into<Point>,
{
    type Output = Self;

    fn sub(self, rhs: P) -> Self::Output {
        self + (-rhs.into())
    }
}
