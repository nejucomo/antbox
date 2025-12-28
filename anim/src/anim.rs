use std::cell::RefCell;

use antbox_gameboard::{BoardState as AntboxState, GenParams};
use antbox_s2render::{RenderScheduler, RenderWithArg};
use antbox_tick_timer::{RateLimiter, TickTimer};
use movestate::TakeIntoNext;
use movestate::next::State;
use speedy2d::Graphics2D;
use speedy2d::dimen::Vec2;

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

    /// Draw `self` onto `gfx`
    pub fn draw(&self, gfx: &mut Graphics2D, view_size: Vec2) {
        let layout = GridLayout::new(self.antbox.bounds(), view_size);

        let mut rsched = self.rs.borrow_mut();
        let mut cycle = rsched.start_cycle();

        cycle.schedule(layers::Background);
        cycle.schedule(spots_into_renderable(&self.antbox, layout, &self.wyrgrid));
        cycle.schedule(layers::WireFrame.with_render_arg(layout));
        cycle.render(gfx);
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
