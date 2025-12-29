use std::cell::RefCell;

use antbox_gameboard::{BoardState as AntboxState, GenParams};
use antbox_render::{RenderCycle, RenderScheduler, RenderWithArg, Renderable};
use antbox_tick_timer::{RateLimiter, TickTimer};
use movestate::TakeIntoNext;
use movestate::next::State;

use crate::layers::Layer;
use crate::{GridLayout, WyrGrid, layers, spots_into_renderable};

const ANTBOX_FRAME_RATE: f64 = 5.0;

/// Encapsulate a [AntboxState] with extra animation-specific state
#[derive(Debug)]
pub struct AnimationState {
    antbox: RateLimiter<AntboxState>,
    wyrgrid: WyrGrid,
    /// This is behind a [RefCell] because we're "caching the allocation"
    rs: RefCell<RenderScheduler>,
}

impl AnimationState {
    /// Initialize
    pub fn new<R: rand::Rng>(rng: &mut R, gp: GenParams) -> Self {
        let antbox = gp.generate_state(rng);
        let antbox = RateLimiter::new(antbox, TickTimer::with_frame_rate(ANTBOX_FRAME_RATE));
        let wyrgrid = WyrGrid::new(antbox.bounds(), rng);
        let rs = RefCell::new(RenderScheduler::new(Layer::count()));

        AnimationState {
            antbox,
            wyrgrid,
            rs,
        }
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
            rs: self.rs,
        }
        .into()
    }
}

impl Renderable for &AnimationState {
    fn schedule(self, cycle: &mut RenderCycle) {
        let layout = GridLayout::new(self.antbox.bounds(), view_size);

        cycle.schedule(layers::Background);
        cycle.schedule(spots_into_renderable(&self.antbox, layout, &self.wyrgrid));
        cycle.schedule(layers::WireFrame.with_render_arg(layout));
        cycle.render(gfx);
    }
}
