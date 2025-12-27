use crate::next::MapState;
use crate::{TakeIntoMapState, TakeIntoNext};

/// The impl of [TakeIntoNext::capture_copy]
#[derive(Clone, Debug)]
pub struct CaptureClone<S, I>
where
    I: Clone,
    S: TakeIntoMapState<I>,
{
    state: S,
    captured: I,
}

impl<S, I> CaptureClone<S, I>
where
    I: Clone,
    S: TakeIntoMapState<I>,
{
    pub(crate) fn new(state: S, captured: I) -> Self {
        Self { state, captured }
    }
}

impl<S, I> TakeIntoNext<()> for CaptureClone<S, I>
where
    I: Clone,
    S: TakeIntoMapState<I>,
{
    type Next = <S::Next as MapState<S>>::MappedState<Self>;

    fn take_into_next(self, (): ()) -> Self::Next {
        let captured = self.captured;
        self.state
            .take_into_next(captured.clone())
            .map_state(|state| Self { state, captured })
    }
}
