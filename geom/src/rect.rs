use derive_more::{From, Into};
use derive_new::new;

use crate::{Bounds, Point};

/// A rectangle
#[derive(Copy, Clone, Debug, From, Into, new)]
pub struct Rect {
    top_left: Point,
    bounds: Bounds,
}
