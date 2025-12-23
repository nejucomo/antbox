use std::marker::PhantomData;

use crate::mutable::Update;
use crate::starg::Starg;
use crate::take_into::TakeIntoNext;

/// An adapter providing a [TakeIntoStarg](crate::take_into::TakeIntoStarg) impl for an inner [Update] type; see [Update::into_take_into_starg]
#[derive(Debug)]
pub struct UpdateAsTakeIntoStarg<T: Update<I, O>, I, O>(T, PhantomData<(I, O)>);

impl<T: Update<I, O>, I, O> UpdateAsTakeIntoStarg<T, I, O> {
    pub(crate) fn new(update: T) -> Self {
        Self(update, PhantomData)
    }
}

impl<T: Update<I, O>, I, O> TakeIntoNext<I> for UpdateAsTakeIntoStarg<T, I, O> {
    type Next = Starg<Self, O>;

    fn take_into_next(mut self, input: I) -> Self::Next {
        let output = self.0.update(input);
        Starg::new(self, output)
    }
}
