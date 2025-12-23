use crate::mutable::UpdateAsTakeIntoStarg;

/// Mutably update and input `I` to produce an `O`
pub trait Update<I, O> {
    /// Mutable update with `input` to produce a `O`
    fn update(&mut self, input: I) -> O;

    /// Convert into a wrapper type which provides [TakeIntoStarg](crate::take_into::TakeIntoStarg)
    fn into_take_into_starg(self) -> UpdateAsTakeIntoStarg<Self, I, O>
    where
        Self: Sized,
    {
        UpdateAsTakeIntoStarg::new(self)
    }
}
