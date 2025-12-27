use rand::Rng;

/// Iterate over interesting states; used for render inspection
pub(crate) trait Interesting: Sized + Copy {
    /// Provide an initial value
    fn first_interesting() -> Self;

    /// Provide a next interesting value
    fn next_interesting<R: Rng>(self, rng: &mut R) -> Option<Self>;
}
