use std::fmt::Debug;

use derive_more::Unwrap;
use movestate::take_into::TakeIntoNext;
use speedy2d::window::{WindowHelper, WindowStartupInfo};

use crate::WindowEventHandler;
use crate::event::WinEvent;

use self::AdapterInner::{Pending, Started};

#[derive(Debug, Unwrap)]
#[unwrap(ref_mut)]
pub(crate) enum AdapterInner<H, U>
where
    U: 'static,
    H: WindowEventHandler<U>,
{
    Pending(H::Params),
    Started(H),
}

impl<H, U> AdapterInner<H, U>
where
    U: 'static,
    H: WindowEventHandler<U>,
{
    pub(crate) fn new(params: H::Params) -> Self {
        Pending(params)
    }
}

impl<'a, H, U> TakeIntoNext<(&'a mut WindowHelper<U>, WindowStartupInfo)> for AdapterInner<H, U>
where
    U: 'static,
    H: WindowEventHandler<U>,
{
    type Next = Self;

    fn take_into_next(
        self,
        (helper, info): (&'a mut WindowHelper<U>, WindowStartupInfo),
    ) -> Self::Next {
        let params = self.unwrap_pending();
        let s = H::start(params, helper, info);
        Started(s)
    }
}

impl<'a, H, U> TakeIntoNext<WinEvent<'a, U>> for AdapterInner<H, U>
where
    U: 'static,
    H: WindowEventHandler<U>,
{
    type Next = Self;

    fn take_into_next(mut self, ev: WinEvent<'a, U>) -> Self::Next {
        self.unwrap_started_mut().update(ev);
        self
    }
}
