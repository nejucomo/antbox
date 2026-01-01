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
    pub(crate) const fn fromp_point(p: Point) -> Self {
        let Point { x, y } = p;
        Dimensions {
            width: Distance::fromp_f32(x),
            height: Distance::fromp_f32(y),
        }
    }
}
