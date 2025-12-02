/// An [Evolvable] tape can produce a new `Self`
pub trait Evolvable: Sized {
    /// Compute the next iteration of `self`
    fn evolve(&self) -> Self;
}
