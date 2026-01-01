use std::fmt::Debug;

use antbox_geom::Dimensions;
use antbox_render::{Backend, RenderWithArg};
use derive_more::Unwrap;
use movestate::TakeIntoNext;
use movestate::next::{State, Stout};
use speedy2d::window::WindowStartupInfo;

use crate::event::WinEvent;
use crate::{Control, UserEventSender, WindowEventHandler};

use self::AdapterInner::{Pending, Started};

#[derive(Debug, Unwrap)]
#[unwrap(ref, ref_mut)]
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

impl<H, U> TakeIntoNext<(UserEventSender<U>, WindowStartupInfo)> for AdapterInner<H, U>
where
    U: 'static,
    H: WindowEventHandler<U>,
{
    type Next = State<Self>;

    fn take_into_next(self, (ues, info): (UserEventSender<U>, WindowStartupInfo)) -> Self::Next {
        let params = self.unwrap_pending();
        let s = H::start(params, ues, info);
        Started(s).into()
    }
}

impl<H, U> TakeIntoNext<WinEvent<U>> for AdapterInner<H, U>
where
    U: 'static,
    H: WindowEventHandler<U>,
{
    type Next = Stout<Self, Control>;

    fn take_into_next(mut self, ev: WinEvent<U>) -> Self::Next {
        let out = self.unwrap_started_mut().update(ev);
        Stout::new(self, out)
    }
}

impl<H, U> RenderWithArg<Dimensions> for &AdapterInner<H, U>
where
    U: 'static,
    H: WindowEventHandler<U>,
{
    fn render_with_arg<B: ?Sized + Backend>(self, rb: &mut B, dims: Dimensions) {
        self.unwrap_started_ref().render_with_arg(rb, dims);
    }
}
