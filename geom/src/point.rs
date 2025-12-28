use std::ops::Add;

use crate::{Angle, Cartesian, Distance, Polar};

/// A point in the two-dimensional plane
pub trait Point: Sized + Copy + Clone + PointPeer<Cartesian> + PointPeer<Polar> {
    /// The x coordinate
    fn x(self) -> f32;

    /// The y coordinate
    fn y(self) -> f32;

    /// The [Angle] from the X-axis in a counterclockwide direction
    fn angle(self) -> Angle;

    /// The [Distance] from the origin
    fn distance(self) -> Distance;
}

/// A supertrait for [Point] to ensure symmetry between Cartesian and Polar representations
pub trait PointPeer<P>: From<P> + Into<P> + Add<P, Output = Self> {}

impl<B, P> PointPeer<P> for B where B: From<P> + Into<P> + Add<P, Output = Self> {}
