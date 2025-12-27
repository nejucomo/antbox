use crate::Field;
use crate::gencount::GenerationCounter;

/// The full [BoardState] is a generation counter and a [Field]
pub type BoardState = GenerationCounter<Field>;
