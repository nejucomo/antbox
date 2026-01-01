use antbox_float::NNF;

use crate::{Angle, Point};

/// Types which can be geometrically transformed
pub trait Transformable: Sized {
    /// Rotate by `angle`
    fn rotate<A>(self, angle: A) -> Self
    where
        A: Into<Angle>,
    {
        self.rotate_by_angle(angle.into())
    }

    /// Rotate by an [Angle]
    fn rotate_by_angle(self, a: Angle) -> Self;

    /// Scale by the `scalar`
    fn scale<F>(self, scalar: F) -> Self
    where
        F: Into<NNF>,
    {
        self.scale_by_nnf(scalar.into())
    }

    /// Scale by an [f32] which panics if [NNF::fromp_f32] panics
    fn scale_by_f32(self, f: f32) -> Self {
        self.scale_by_nnf(NNF::fromp_f32(f))
    }

    /// Scale by an [NNF]
    fn scale_by_nnf(self, s: NNF) -> Self;

    /// Translate by `delta`
    fn translate<P>(self, delta: P) -> Self
    where
        P: Into<Point>,
    {
        self.translate_by_point(delta.into())
    }

    /// Translate by a [Point]
    fn translate_by_point(self, p: Point) -> Self;
}
