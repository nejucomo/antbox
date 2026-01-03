use crate::IntoNext;
use crate::next::MapState;

/// `S -> F S`: Any conversion into a parameterization over `Self`
pub trait IntoMapState: IntoNext<Next: MapState<Self>> {}

impl<B> IntoMapState for B where B: IntoNext<Next: MapState<B>> {}
