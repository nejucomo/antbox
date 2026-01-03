use derive_more::Deref;
use derive_new::new;
use mstate::MStateIn;

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

impl<S, I> MStateIn<I> for GenerationCounter<S>
where
    S: MStateIn<(GenerationCount, I), Next: Into<S>>,
{
    type Next = Self;

    fn into_with(self, input: I) -> Self::Next {
        let GenerationCounter { gc, inner } = self;

        let inner = inner.into_with((gc, input)).into();
        let gc = GenerationCount(gc.0 + 1);
        GenerationCounter { gc, inner }
    }
}

impl<S> MStateIn<GenerationCount> for Cycler<S>
where
    S: MStateIn<(), Next: Into<S>>,
{
    type Next = Self;

    fn into_with(self, gc: GenerationCount) -> Self::Next {
        self.into_with((gc, ()))
    }
}

impl<S, I> MStateIn<(GenerationCount, I)> for Cycler<S>
where
    S: MStateIn<I, Next: Into<S>>,
{
    type Next = Self;

    fn into_with(self, (gc, input): (GenerationCount, I)) -> Self::Next {
        let inner = if gc.is_multiple_of(self.interval) {
            self.inner.into_with(input).into()
        } else {
            self.inner
        };
        Cycler { inner, ..self }
    }
}
