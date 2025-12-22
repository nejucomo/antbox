//! Types which mutably *update* themselves

/// Mutably update with an input `I`
pub trait Update<I> {
    /// Update `self` with `input`
    fn update(&mut self, input: I);
}
