//! A trait family for types which move `self` without input _into_ a [Next](crate::takeinto::TakeIntoNext::Next) type

mod next;
mod optupdatewith;
mod updatewith;
mod upout;

pub use self::next::IntoNext;
pub use self::optupdatewith::IntoOptUpdateWith;
pub use self::updatewith::IntoUpdateWith;
pub use self::upout::IntoUpdateWithOutput;
