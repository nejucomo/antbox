use crate::TakeIntoNext;
use crate::next::MapState;

/// The impl of [TakeIntoNext::capture_copy]
#[derive(Copy, Clone, Debug)]
pub struct CaptureCopy<S, I>
where
    I: Copy,
    S: TakeIntoNext<I, Next: MapState<S>>,
{
    state: S,
    captured: I,
}

impl<S, I> CaptureCopy<S, I>
where
    I: Copy,
    S: TakeIntoNext<I, Next: MapState<S>>,
{
    pub(crate) fn new(state: S, captured: I) -> Self {
        Self { state, captured }
    }
}

impl<S, I> TakeIntoNext<()> for CaptureCopy<S, I>
where
    I: Copy,
    S: TakeIntoNext<I, Next: MapState<S>>,
{
    type Next = <S::Next as MapState<S>>::MappedState<Self>;

    fn take_into_next(self, (): ()) -> Self::Next {
        let captured = self.captured;
        self.state
            .take_into_next(captured)
            .map_state(|state| Self { state, captured })
    }
}
