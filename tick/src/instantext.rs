use std::time::Instant;

use extension_traits::extension;

use crate::TargetDelta;

#[extension(pub trait InstantExt)]
impl Instant {
    /// The delta from now to `self` as a target
    fn delta_from_now(self) -> TargetDelta {
        TargetDelta::new(Instant::now(), self)
    }
}
