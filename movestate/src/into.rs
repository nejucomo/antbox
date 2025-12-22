//! A trait family for types which move `self` _into_ a [Next](IntoNextWith::Next) type, built around [IntoNextWith]

mod next;
mod nextwith;
mod optupdate;
mod updatewith;
mod upout;

pub use self::next::IntoNext;
pub use self::nextwith::IntoNextWith;
pub use self::optupdate::IntoOptUpdate;
pub use self::updatewith::IntoUpdateWith;
pub use self::upout::IntoUpdateWithOutput;
