use crate::Generation;

/// A [Ruleset] can compute a new [Generation] from any given [Generation]
pub trait Ruleset {
    /// Compute the next [Generation] from `g`
    fn next_generation(&self, g: &Generation) -> Generation;
}
