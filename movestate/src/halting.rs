//! `* -> [*]`: The [Halting] `Next` type

mod fromimpls;
mod hstate;
mod hstout;

use derive_more::From;

use self::Halting::*;

pub use self::hstate::{IntoHaltingState, TakeIntoHaltingState};
pub use self::hstout::{IntoHaltingStout, TakeIntoHaltingStout};

/// `[S]` / `[S, O]`
///
/// Either a `Next` state `N` or a [Halt] value
#[derive(Copy, Clone, Debug, From)]
pub enum Halting<N> {
    /// Continue to the next state `N`
    Continue(N),
    /// Halt transitions
    ///
    /// # Note
    ///
    /// This value implies the initial state has been dropped.
    Halt,
}

impl<N> Halting<N> {
    fn from_option(opt: Option<N>) -> Self {
        opt.map(Continue).unwrap_or(Halt)
    }

    fn into_option(self) -> Option<N> {
        match self {
            Continue(s) => Some(s),
            Halt => None,
        }
    }
}
