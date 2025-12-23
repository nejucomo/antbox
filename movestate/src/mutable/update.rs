/// Mutably update and input `I` to produce an [Self::Output]
pub trait Update<I> {
    /// The output produced
    type Output;

    /// Mutable update with `input` to produce a [Self::Output]
    fn update(&mut self, input: I) -> Self::Output;
}
