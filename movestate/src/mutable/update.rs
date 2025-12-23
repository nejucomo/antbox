use crate::mutable::UpdateAsTakeIntoNext;

/// Mutably update and input `I` to produce an `O`
pub trait Update<I, O> {
    /// Mutable update with `input` to produce a `O`
    fn update(&mut self, input: I) -> O;

    /// Convert into a wrapper type which provides [TakeIntoNext](crate::TakeIntoNext)
    fn into_take_into_starg(self) -> UpdateAsTakeIntoNext<Self, I, O>
    where
        Self: Sized,
    {
        UpdateAsTakeIntoNext::new(self)
    }
}
