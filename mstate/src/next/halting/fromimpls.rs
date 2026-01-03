mod from_for_halting {
    mod state {
        use crate::next::{Halting, State};

        impl<S> From<Option<State<S>>> for Halting<State<S>> {
            fn from(opt: Option<State<S>>) -> Self {
                Self::from_option(opt)
            }
        }

        impl<S> From<Option<S>> for Halting<State<S>> {
            fn from(opt: Option<S>) -> Self {
                opt.map(State::new).into()
            }
        }
    }

    mod stout {
        use crate::next::{Halting, Stout};

        impl<S, O> From<Option<Stout<S, O>>> for Halting<Stout<S, O>> {
            fn from(opt: Option<Stout<S, O>>) -> Self {
                Self::from_option(opt)
            }
        }

        impl<S, O> From<Option<(S, O)>> for Halting<Stout<S, O>> {
            fn from(opt: Option<(S, O)>) -> Self {
                opt.map(Stout::from).into()
            }
        }
    }
}

mod from_halting_for_foreign {
    mod state {
        use crate::next::{Halting, State};

        impl<S> From<Halting<State<S>>> for Option<State<S>> {
            fn from(h: Halting<State<S>>) -> Self {
                h.into_option()
            }
        }

        impl<S> From<Halting<State<S>>> for Option<S> {
            fn from(h: Halting<State<S>>) -> Self {
                h.into_option().map(|s| s.state)
            }
        }
    }

    mod stout {
        use crate::next::{Halting, Stout};

        impl<S, O> From<Halting<Stout<S, O>>> for Option<Stout<S, O>> {
            fn from(h: Halting<Stout<S, O>>) -> Self {
                h.into_option()
            }
        }

        impl<S, O> From<Halting<Stout<S, O>>> for Option<(S, O)> {
            fn from(h: Halting<Stout<S, O>>) -> Self {
                h.into_option().map(|s| s.into())
            }
        }
    }
}
