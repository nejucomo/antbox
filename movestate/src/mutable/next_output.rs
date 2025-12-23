use crate::mutable::Update;

/// Mutably produce a sequence of [Output](Update::Output)s
pub trait NextOutput: Update<()> {
    /// Mutably produce the next [Output](Update::Output)
    fn next_output(&mut self) -> Self::Output {
        self.update(())
    }
}

impl<B> NextOutput for B where B: Update<()> {}
