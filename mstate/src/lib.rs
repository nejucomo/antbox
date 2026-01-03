// #![deny(unsafe_code, missing_docs)]
#![deny(unsafe_code)]

use moveslot::{MapInPlace as _, MoveSlot};

pub trait MStateIn<I>: Sized {
    type Next;

    fn into_with(self, input: I) -> Self::Next;
}

pub trait MState: MStateIn<()> {
    fn into_next(self) -> Self::Next;
}

impl<B> MState for B
where
    B: MStateIn<()>,
{
    fn into_next(self) -> Self::Next {
        self.into_with(())
    }
}

pub trait Update<I> {
    fn update(&mut self, notification: I);
}

impl<T, I> Update<I> for MoveSlot<T>
where
    T: MStateIn<I, Next = T>,
{
    fn update(&mut self, input: I) {
        self.map_in_place(|t| t.into_with(input))
    }
}

pub trait Responder<I> {
    type Response;

    fn handle(&mut self, request: I) -> Self::Response;
}

impl<T, I, O> Responder<I> for MoveSlot<T>
where
    T: MStateIn<I, Next = (T, O)>,
{
    type Response = O;

    fn handle(&mut self, input: I) -> Self::Response {
        self.mip_out(|t| t.into_with(input))
    }
}
