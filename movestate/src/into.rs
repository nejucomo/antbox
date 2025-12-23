//! A trait family for types which move `self` without input _into_ a [Next](crate::takeinto::TakeIntoNext::Next) type

mod next;

pub use self::next::IntoNext;
