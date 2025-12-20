use antbox_geom::Grid;
use antbox_state::{GenParams, State as AntboxState};
use antbox_tick_timer::{RateLimiter, TickTimer};
use movestate::{Transform, Update as _};
use speedy2d::Graphics2D;
use speedy2d::dimen::Vec2;
use wyrand::WyRand;

use crate::{Drawable as _, GridLayout, layers};

const ANTBOX_FRAME_RATE: f64 = 1.0;

/// Encapsulate a [AntboxState] with extra animation-specific state
#[derive(Debug)]
pub struct AnimationState {
    antbox: RateLimiter<AntboxState>,
    wyrgrid: Grid<WyRand>,
}

impl AnimationState {
    /// Initialize
    pub fn new<R: rand::Rng>(rng: &mut R, gp: GenParams) -> Self {
        let antbox = gp.generate_state(rng);
        let antbox = RateLimiter::new(antbox, TickTimer::with_frame_rate(ANTBOX_FRAME_RATE));
        let bounds = antbox.bounds();
        let mut v = Vec::with_capacity(bounds.area());
        for _ in 0..bounds.area() {
            v.push(WyRand::new(rng.random()));
        }

        AnimationState {
            antbox,
            wyrgrid: Grid::new(bounds, v),
        }
    }

    /// Draw `self` onto `gfx`
    pub fn draw(&self, gfx: &mut Graphics2D, view_size: Vec2) {
        let layout = GridLayout::new(self.antbox.bounds(), view_size);
        layers::Background.draw_on(gfx, ());
        layers::WireFrame.draw_on(gfx, layout);
        self.antbox.draw_on(gfx, (layout, &self.wyrgrid));
    }
}

impl<R> Transform<&mut R> for AnimationState
where
    R: rand::Rng,
{
    type Next = Self;

    fn transform(self, rng: &mut R) -> Self {
        AnimationState {
            antbox: self.antbox.update(rng),
            wyrgrid: self.wyrgrid,
        }
    }
}
