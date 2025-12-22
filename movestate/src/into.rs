//! Types which move _into_ a next type

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
