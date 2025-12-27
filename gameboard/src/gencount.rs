use derive_more::Deref;
use derive_new::new;
use movestate::TakeIntoNext;

#[derive(Debug, new, Deref)]
pub struct GenerationCounter<S> {
    #[new(default)]
    gc: GenerationCount,
    #[deref]
    inner: S,
}

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct GenerationCount(usize);

#[derive(Debug, new, Deref)]
pub(crate) struct Cycler<S> {
    #[deref]
    inner: S,
    interval: usize,
}

impl GenerationCount {
    pub(crate) fn is_multiple_of(self, other: usize) -> bool {
        self.0.is_multiple_of(other)
    }
}

impl<S, I> TakeIntoNext<I> for GenerationCounter<S>
where
    S: TakeIntoNext<(GenerationCount, I), Next: Into<S>>,
{
    type Next = Self;

    fn take_into_next(self, input: I) -> Self::Next {
        let GenerationCounter { gc, inner } = self;

        let inner = inner.take_into_next((gc, input)).into();
        let gc = GenerationCount(gc.0 + 1);
        GenerationCounter { gc, inner }
    }
}

impl<S> TakeIntoNext<GenerationCount> for Cycler<S>
where
    S: TakeIntoNext<(), Next: Into<S>>,
{
    type Next = Self;

    fn take_into_next(self, gc: GenerationCount) -> Self::Next {
        self.take_into_next((gc, ()))
    }
}

impl<S, I> TakeIntoNext<(GenerationCount, I)> for Cycler<S>
where
    S: TakeIntoNext<I, Next: Into<S>>,
{
    type Next = Self;

    fn take_into_next(self, (gc, input): (GenerationCount, I)) -> Self::Next {
        let inner = if gc.is_multiple_of(self.interval) {
            self.inner.take_into_next(input).into()
        } else {
            self.inner
        };
        Cycler { inner, ..self }
    }
}
