use crate::mutable::Update;

/// Mutably produce a sequence of output `O` values
pub trait NextOutput<O>: Update<(), O> {
    /// Mutably produce the next `O`
    fn next_output(&mut self) -> O {
        self.update(())
    }
}

impl<B, O> NextOutput<O> for B where B: Update<(), O> {}
