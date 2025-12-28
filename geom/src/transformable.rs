use crate::{Angle, Point};

/// Types which can be geometrically transformed
pub trait Transformable: Sized + Copy {
    /// Rotate by an [Angle]
    fn rotate<A>(self, a: A) -> Self
    where
        A: Into<Angle>;

    /// Scale by the scalar `s`
    fn scale(self, s: f32) -> Self;

    /// Translate by `delta`
    fn translate(self, delta: Point) -> Self;
}
