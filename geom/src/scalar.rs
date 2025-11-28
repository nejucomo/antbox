use std::num::TryFromIntError;

use num_integer::Integer;

/// The trait required for the parameter to the other geometry types
pub trait Scalar: Copy + Integer + TryFrom<usize, Error = TryFromIntError>
where
    usize: TryFrom<Self, Error = TryFromIntError>,
{
}

impl<S> Scalar for S
where
    S: Copy + Integer + TryFrom<usize, Error = TryFromIntError>,
    usize: TryFrom<Self, Error = TryFromIntError>,
{
}
