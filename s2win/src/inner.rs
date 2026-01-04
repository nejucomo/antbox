use std::fmt::Debug;

use antbox_geom::Dimensions;
use antbox_render::{Backend, RenderWithArg};
use derive_more::Unwrap;
use mstate::{MStateIn, Responder};
use speedy2d::window::WindowStartupInfo;

use crate::event::InitEvent;
use crate::{Control, S2App, UserEventSender};

use self::AdapterInner::{Pending, Started};

#[derive(Debug, Unwrap)]
#[unwrap(ref, ref_mut)]
pub(crate) enum AdapterInner<A>
where
    A: S2App,
    A: S2App,
{
    Pending(A::Params),
    Started(A),
}

impl<A> AdapterInner<A>
where
    A: S2App,
{
    pub(crate) fn new(params: A::Params) -> Self {
        Pending(params)
    }

    pub(crate) fn start_transition(
        self,
        ues: UserEventSender<A::Event>,
        info: WindowStartupInfo,
    ) -> Self {
        let params = self.unwrap_pending();
        Started(A::from(InitEvent { params, ues, info }))
    }
}

impl<A, V> MStateIn<V> for AdapterInner<A>
where
    A: S2App + Responder<V, Response = Control>,
{
    type Next = (Self, Control);

    fn into_with(mut self, ev: V) -> Self::Next {
        let out = self.unwrap_started_mut().handle(ev);
        (self, out)
    }
}

impl<A> RenderWithArg<Dimensions> for &AdapterInner<A>
where
    A: S2App,
{
    fn render_with_arg<B: ?Sized + Backend>(self, rb: &mut B, dims: Dimensions) {
        self.unwrap_started_ref().render_with_arg(rb, dims);
    }
}
