/// Mutably update and input `I` to produce an `O`
pub trait Update<I, O> {
    /// Mutable update with `input` to produce a `O`
    fn update(&mut self, input: I) -> O;
}
