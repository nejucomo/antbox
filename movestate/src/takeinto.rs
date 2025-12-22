//! A trait family for types which move `self` and an input _into_ a [Next](crate::takeinto::TakeIntoNext::Next) type

mod next;

pub use self::next::TakeIntoNext;
