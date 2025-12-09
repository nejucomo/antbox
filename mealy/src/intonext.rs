/// A self-iterative process where each state derives the next
///
/// Every [IntoNext] type is an `UpdateInput<()>` type by a blanket impl.
pub trait IntoNext: Sized {
    /// Convert the current state into the next state
    fn into_next(self) -> Self;
}
