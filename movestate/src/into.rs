//! A trait family for types which move `self` _into_ a [Next](TakeIntoNext::Next) type, built around [TakeIntoNext]

mod next;
mod optupdatewith;
mod updatewith;
mod upout;

pub mod take;
pub use self::next::IntoNext;
pub use self::optupdatewith::IntoOptUpdateWith;
pub use self::updatewith::IntoUpdateWith;
pub use self::upout::IntoUpdateWithOutput;
