use crate::TakeIntoNext;
use crate::next::MapState;

/// `(S, I) -> F S`: Any conversion with input into a parameterization over `Self`
pub trait TakeIntoMapState<I>: TakeIntoNext<I, Next: MapState<Self>> {}

impl<B, I> TakeIntoMapState<I> for B where B: TakeIntoNext<I, Next: MapState<B>> {}
