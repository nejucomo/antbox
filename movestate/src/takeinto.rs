//! A trait family for types which move `self` and an input _into_ a [Next](crate::takeinto::TakeIntoNext::Next) type

mod next;
mod optupdate;
mod update;
mod updateout;

pub use self::next::TakeIntoNext;
pub use self::optupdate::TakeIntoOptUpdate;
pub use self::update::TakeIntoUpdate;
pub use self::updateout::TakeIntoUpdateOut;
