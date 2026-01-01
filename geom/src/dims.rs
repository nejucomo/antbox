use std::num::NonZero;
use std::ops::{Div, Mul};

use derive_more::{From, Into};
use derive_new::new;

use crate::{Distance, Point};

/// Width and height dimensions
#[derive(Copy, Clone, Debug, new, Into, From)]
pub struct Dimensions {
    #[allow(missing_docs)]
    pub width: Distance,
    #[allow(missing_docs)]
    pub height: Distance,
}

impl Dimensions {
    /// Convert to a [Point] at `self.width` and `self.height` from the [Point::ORIGIN]
    pub fn into_bottom_right(self) -> Point {
        Point::new(self.width.into(), self.height.into())
    }

    pub(crate) const fn fromp_point(p: Point) -> Self {
        let Point { x, y } = p;
        Dimensions {
            width: Distance::fromp_f32(x),
            height: Distance::fromp_f32(y),
        }
    }
}

impl Mul<(usize, usize)> for Dimensions {
    type Output = Self;

    fn mul(self, (col, row): (usize, usize)) -> Self::Output {
        Dimensions {
            width: self.width * col,
            height: self.height * row,
        }
    }
}

impl Div<(NonZero<usize>, NonZero<usize>)> for Dimensions {
    type Output = Self;

    fn div(self, (cols, rows): (NonZero<usize>, NonZero<usize>)) -> Self::Output {
        Dimensions {
            width: self.width / cols,
            height: self.height / rows,
        }
    }
}
