/// An [Evolvable] tape can produce a new `Self`
///
/// # TODO
///
/// Replace this by extending `mealy_machine` appropriately
pub trait Evolvable: Sized {
    /// Compute the next iteration of `self`
    fn evolve(&self) -> Self;
}
