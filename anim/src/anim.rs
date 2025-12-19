use antbox_state::{GenParams, State as AntboxState};
use movestate::toolkit::Cycler;
use movestate::{Transform, Update as _};
use speedy2d::Graphics2D;
use speedy2d::dimen::Vec2;

use crate::{GfxLayout, GridLayout, layers};

const ANIMS_PER_STATE_TICK: usize = 13;

/// Encapsulate a [AntboxState] with extra animation-specific state
#[derive(Debug)]
pub struct AnimationState {
    antbox: Cycler<AntboxState>,
}

impl AnimationState {
    /// Initialize
    pub fn new<R: rand::Rng>(rng: &mut R, gp: GenParams) -> Self {
        let antbox = Cycler::new(gp.generate_state(rng), ANIMS_PER_STATE_TICK);
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
