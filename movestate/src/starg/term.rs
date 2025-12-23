use derive_more::From;

use crate::starg::Starg;

use TermStarg::{Starg as StargV, Terminal};

/// A hack work-around for bug in `#[from((A, B))]` parsing
type Pair<A, B> = (A, B);

/// Either a `Starg<S, A>` or a terminal value `T`
#[derive(Debug, From)]
pub enum TermStarg<S, A, T> {
    /// A [Starg]
    #[from(Starg<S, A>)]
    #[from(Pair<S, A>)]
    Starg(Starg<S, A>),
    /// A terminal value, `T`
    Terminal(T),
}

/// An isomophism to `Option<Starg<S, A>>` via [TermStarg] with terminal `()`
pub type OptStarg<S, A> = TermStarg<S, A, ()>;

/// A [TermStarg] with a [Result]`<(), E>` terminal
pub type ResStarg<S, A, E> = TermStarg<S, A, Result<(), E>>;

mod opt_starg_conversions {
    use super::*;

    impl<S, A> From<Option<(S, A)>> for OptStarg<S, A> {
        fn from(opt: Option<(S, A)>) -> Self {
            opt.map(Starg::from).into()
        }
    }

    impl<S, A> From<OptStarg<S, A>> for Option<(S, A)> {
        fn from(ts: OptStarg<S, A>) -> Self {
            let opt: Option<Starg<S, A>> = ts.into();
            opt.map(|ts: Starg<S, A>| ts.into())
        }
    }

    impl<S, A> From<Option<Starg<S, A>>> for OptStarg<S, A> {
        fn from(opt: Option<Starg<S, A>>) -> Self {
            opt.map(StargV).unwrap_or(Terminal(()))
        }
    }

    impl<S, A> From<OptStarg<S, A>> for Option<Starg<S, A>> {
        fn from(ts: OptStarg<S, A>) -> Self {
            match ts {
                StargV(starg) => Some(starg),
                Terminal(()) => None,
            }
        }
    }
}

mod res_starg_conversions {
    use super::*;

    impl<S, A, E> From<Result<(S, A), E>> for ResStarg<S, A, E> {
        fn from(res: Result<(S, A), E>) -> Self {
            res.map(Some).into()
        }
    }

    impl<S, A, E> From<Result<Starg<S, A>, E>> for ResStarg<S, A, E> {
        fn from(res: Result<Starg<S, A>, E>) -> Self {
            res.map(Some).into()
        }
    }

    impl<S, A, E> From<Result<Option<(S, A)>, E>> for ResStarg<S, A, E> {
        fn from(rop: Result<Option<(S, A)>, E>) -> Self {
            rop.map(|op| op.map(Starg::from)).into()
        }
    }

    impl<S, A, E> From<Result<Option<Starg<S, A>>, E>> for ResStarg<S, A, E> {
        fn from(rostarg: Result<Option<Starg<S, A>>, E>) -> Self {
            match rostarg {
                Ok(Some(st)) => StargV(st),
                Ok(None) => Terminal(Ok(())),
                Err(e) => Terminal(Err(e)),
            }
        }
    }

    impl<S, A, E> From<ResStarg<S, A, E>> for Result<Option<Starg<S, A>>, E> {
        fn from(rs: ResStarg<S, A, E>) -> Self {
            match rs {
                StargV(st) => Ok(Some(st)),
                Terminal(res) => res.map(|()| None),
            }
        }
    }
}
