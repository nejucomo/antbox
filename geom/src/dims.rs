use derive_more::{From, Into};
use derive_new::new;

use crate::Distance;

/// Width and height dimensions
#[derive(Copy, Clone, Debug, new, Into, From)]
pub struct Dimensions {
    #[allow(missing_docs)]
    pub width: Distance,
    #[allow(missing_docs)]
    pub height: Distance,
}
