/// Functor-like types which can map a state `S` to a new state
pub trait MapState<S> {
    /// The result of mapping the state
    ///
    /// To approximate `Functor`-like behavior, impl types are parameterized by `S`, and then set `MappedState<MS> = Self<MS, ...>`.
    type MappedState<MS>;

    /// Map our state `S` into a new self parameterized by state `f(s)`
    fn map_state<F, T>(self, f: F) -> Self::MappedState<T>
    where
        F: FnOnce(S) -> T;
}
