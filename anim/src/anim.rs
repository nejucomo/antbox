use antbox_state::{GenParams, State as AntboxState};
use antbox_tick_timer::{RateLimiter, TickTimer};
use movestate::{Transform, Update as _};
use speedy2d::Graphics2D;
use speedy2d::dimen::Vec2;

use crate::{GfxLayout, GridLayout, layers};

const ANTBOX_FRAME_RATE: f64 = 1.0;

/// Encapsulate a [AntboxState] with extra animation-specific state
#[derive(Debug)]
pub struct AnimationState {
    antbox: RateLimiter<AntboxState>,
}

impl AnimationState {
    /// Initialize
    pub fn new<R: rand::Rng>(rng: &mut R, gp: GenParams) -> Self {
        let antbox = gp.generate_state(rng);
        let antbox = RateLimiter::new(antbox, TickTimer::with_frame_rate(ANTBOX_FRAME_RATE));
        AnimationState { antbox }
    }

    /// Draw `self` onto `gfx`
    pub fn draw(&self, g: &mut Graphics2D, view_size: Vec2) {
        let gl = GridLayout::new(self.antbox.bounds(), view_size);
        let mut gfx = GfxLayout::new(g, gl);

        gfx.draw(layers::Background);
        gfx.draw(layers::WireFrame);
        gfx.draw(&*self.antbox);
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
        }
    }
}
