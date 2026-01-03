use std::fmt::Debug;

use antbox_geom::Dimensions;
use antbox_render::{Backend, RenderWithArg};
use derive_more::Unwrap;
use mstate::MStateIn;
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

impl<H, U> MStateIn<(UserEventSender<U>, WindowStartupInfo)> for AdapterInner<H, U>
where
    U: 'static,
    H: WindowEventHandler<U>,
{
    type Next = Self;

    fn into_with(self, (ues, info): (UserEventSender<U>, WindowStartupInfo)) -> Self::Next {
        let params = self.unwrap_pending();
        Started(H::start(params, ues, info))
    }
}

impl<H, U> MStateIn<WinEvent<U>> for AdapterInner<H, U>
where
    U: 'static,
    H: WindowEventHandler<U>,
{
    type Next = (Self, Control);

    fn into_with(mut self, ev: WinEvent<U>) -> Self::Next {
        let out = self.unwrap_started_mut().handle(ev);
        (self, out)
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
