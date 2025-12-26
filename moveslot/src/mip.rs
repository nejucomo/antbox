/// Wrapper types which can own/replace `T` within closures behing `&mut self`
pub trait MapInPlace<T>: From<T> {
    /// Unwrap the current state; requiring a valid state in `self`
    ///
    /// # Panics
    ///
    /// Panics if there is no stored state.
    fn unwrap_state(self) -> T;

    /// Unwrap to an optional state
    fn opt_state(self) -> Option<T>;

    /// Replace the stored value by transforming it through a mapping `f`
    ///
    /// # Panics
    ///
    /// Panics if there is no stored state.
    fn map_in_place<F>(&mut self, f: F)
    where
        F: FnOnce(T) -> T,
    {
        self.mip_out(|x| (f(x), ()));
    }

    /// <u>M</u>ap <u>i</u>n <u>p</u>lace producing output; similar to [Self::map_in_place] except `f` also produces an output
    ///
    /// # Panics
    ///
    /// Panics if there is no stored state.
    fn mip_out<F, O>(&mut self, f: F) -> O
    where
        F: FnOnce(T) -> (T, O);

    /// <u>M</u>ap <u>i</u>n <u>p</u>lace, <u>opt</u>ionally
    ///
    /// If the mapped value is `None`, then `self` no longer stores a valid state.
    fn mip_out_opt<F, O>(&mut self, f: F) -> Option<O>
    where
        F: FnOnce(T) -> Option<(T, O)>;

    /// <u>M</u>ap <u>i</u>n <u>p</u>lace with a fallible <u>res</u>ult
    ///
    /// If the mapped value is `None`, then `self` no longer stores a valid state.
    fn mip_out_res<F, O, E>(&mut self, f: F) -> Result<O, E>
    where
        F: FnOnce(T) -> Result<(T, O), E>;
}
