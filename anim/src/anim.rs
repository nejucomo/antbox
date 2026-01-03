use antbox_gameboard::{BoardState as AntboxState, GenParams};
use antbox_geom::Dimensions;
use antbox_render::{Backend, RenderRefWithArg, RenderWithArg, Renderable};
use antbox_tick_timer::{RateLimiter, TickTimer};
use mstate::TakeIntoNext;
use mstate::next::State;

use crate::{GridLayout, WyrGrid, layers, spots_into_renderable};

const ANTBOX_FRAME_RATE: f64 = 5.0;

/// Encapsulate a [AntboxState] with extra animation-specific state
#[derive(Debug)]
pub struct AnimationState {
    antbox: RateLimiter<AntboxState>,
    wyrgrid: WyrGrid,
}

impl AnimationState {
    /// Initialize
    pub fn new<R: rand::Rng>(rng: &mut R, gp: GenParams) -> Self {
        let antbox = gp.generate_state(rng);
        let antbox = RateLimiter::new(antbox, TickTimer::with_frame_rate(ANTBOX_FRAME_RATE));
        let wyrgrid = WyrGrid::new(antbox.bounds(), rng);

        AnimationState { antbox, wyrgrid }
    }
}

impl<R> TakeIntoNext<&mut R> for AnimationState
where
    R: rand::Rng,
{
    type Next = State<Self>;

    fn take_into_next(self, rng: &mut R) -> Self::Next {
        AnimationState {
            antbox: self.antbox.take_into_next(rng),
            wyrgrid: self.wyrgrid,
        }
        .into()
    }
}

impl RenderRefWithArg<Dimensions> for AnimationState {
    fn render_ref_with_arg<B: ?Sized + Backend>(&self, rb: &mut B, view_size: Dimensions) {
        let layout = GridLayout::new(self.antbox.bounds(), view_size);

        (
            layers::Background,
            spots_into_renderable(&self.antbox, layout, &self.wyrgrid),
            layers::WireFrame.with_render_arg(layout),
        )
            .render_to(rb);
    }
}
