use std::marker::PhantomData;

use derive_new::new;

use crate::TakeIntoNext;
use crate::mutable::Update;
use crate::starg::Starg;

/// An adapter providing a [TakeIntoNext] impl for an inner [Update] type; see [Update::into_take_into_starg]
#[derive(Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct UpdateAsTakeIntoNext<T: Update<I, O>, I, O>(T, #[new(default)] PhantomData<(I, O)>);

impl<T: Update<I, O>, I, O> TakeIntoNext<I> for UpdateAsTakeIntoNext<T, I, O> {
    type Next = Starg<Self, O>;

    fn take_into_next(mut self, input: I) -> Self::Next {
        let output = self.0.update(input);
        Starg::new(self, output)
    }
}
