/// Wrapper types which can own/replace `T` within closures behing `&mut self`
pub trait MapInPlace<T>: From<T> {
    /// Unwrap the slot
    fn unwrap_mip(self) -> T;

    /// Replace the stored value by transforming it through a mapping `f`
    fn map_in_place<F>(&mut self, f: F)
    where
        F: FnOnce(T) -> T,
    {
        self.mapout_in_place(|x| (f(x), ()));
    }

    /// Similar to [Self::map_in_place] except `f` also produces an output
    fn mapout_in_place<F, O>(&mut self, f: F) -> O
    where
        F: FnOnce(T) -> (T, O);
}
