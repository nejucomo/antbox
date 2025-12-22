use std::fmt::Debug;

use derive_more::Unwrap;
use movestate::into::IntoNextWith;
use speedy2d::window::{WindowHelper, WindowStartupInfo};

use crate::WindowEventHandler;
use crate::event::WinEvent;

use self::AdapterInner::{Pending, Started};

#[derive(Debug, Unwrap)]
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

impl<'a, H, U> IntoNextWith<(&'a mut WindowHelper<U>, WindowStartupInfo)> for AdapterInner<H, U>
where
    U: 'static,
    H: WindowEventHandler<U>,
{
    type Next = Self;

    fn into_next_with(
        self,
        (helper, info): (&'a mut WindowHelper<U>, WindowStartupInfo),
    ) -> Self::Next {
        let params = self.unwrap_pending();
        let s = H::start(params, helper, info);
        Started(s)
    }
}

impl<'a, H, U> IntoNextWith<WinEvent<'a, U>> for AdapterInner<H, U>
where
    U: 'static,
    H: WindowEventHandler<U>,
{
    type Next = Self;

    fn into_next_with(self, ev: WinEvent<'a, U>) -> Self::Next {
        let s = self.unwrap_started();
        Started(s.into_update_out(ev))
    }
}
