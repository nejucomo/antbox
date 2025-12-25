use crate::halting::Halting;
use crate::stout::Stout;
use crate::{IntoNext, TakeIntoNext};

/// `(S, I) -> [S, O]`
pub trait TakeIntoHaltingStout<I, O>:
    Sized + TakeIntoNext<I, Next: Into<Halting<Stout<Self, O>>>>
{
    /// `(S, I) -> [S, O]`
    fn take_into_opt_self_out(self, input: I) -> Option<(Self, O)> {
        self.take_into_hstout(input).into()
    }

    /// Transition `self` and an `input` into a [Halting] [Stout]
    ///
    /// Consumers typically use [TakeIntoHaltingStout::take_into_opt_self_out] for ergonomics.
    fn take_into_hstout(self, input: I) -> Halting<Stout<Self, O>> {
        self.take_into_next(input).into()
    }
}

/// `S -> [S, O]`
pub trait IntoHaltingStout<O>:
    IntoNext + TakeIntoNext<(), Next: Into<Halting<Stout<Self, O>>>>
{
    /// `S -> [S, O]`
    fn into_opt_self_out(self) -> Option<(Self, O)> {
        self.into_hstout().into()
    }

    /// Transition `self` into a [Halting] [Stout]
    ///
    /// Consumers typically use [IntoHaltingStout::into_opt_self_out] for ergonomics.
    fn into_hstout(self) -> Halting<Stout<Self, O>> {
        self.into_next().into()
    }
}

mod blanket_extensions {
    use crate::halting::{Halting, IntoHaltingStout, TakeIntoHaltingStout};
    use crate::stout::Stout;
    use crate::{IntoNext, TakeIntoNext};

    impl<B, I, O> TakeIntoHaltingStout<I, O> for B where
        B: Sized + TakeIntoNext<I, Next: Into<Halting<Stout<Self, O>>>>
    {
    }

    impl<B, O> IntoHaltingStout<O> for B where
        B: IntoNext + TakeIntoNext<(), Next: Into<Halting<Stout<Self, O>>>>
    {
    }
}
