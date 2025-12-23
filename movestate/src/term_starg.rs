use crate::Starg;

/// Either a `Starg<S, A>` or a terminal value `T`
#[derive(Debug)]
pub enum TermStarg<S, A, T> {
    /// A [Starg]
    Starg(Starg<S, A>),
    /// A terminal value, `T`
    Terminal(T),
}
