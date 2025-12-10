use antbox_state::{GenParams, State as AntboxState};
use mealy_machine::{IntoNext, UpdateInput};
use speedy2d::Graphics2D;
use speedy2d::dimen::Vec2;

use crate::{GfxLayout, GridLayout, TICKS_PER_CONWAY, UpdateCycler, layers};

/// Encapsulate a [AntboxState] with extra animation-specific state
#[derive(Debug)]
pub struct AnimationState {
    antbox: UpdateCycler<AntboxState>,
    food: layers::Food,
}

impl AnimationState {
    /// Initialize
    pub fn new<R: rand::Rng>(rng: &mut R, gp: GenParams) -> Self {
        let antbox = gp.generate_state(rng);
        let bounds = antbox.bounds;
        AnimationState {
            antbox: UpdateCycler::new(antbox, TICKS_PER_CONWAY),
            food: layers::Food::from(bounds),
        }
    }

    /// Draw `self` onto `gfx`
    pub fn draw(&self, g: &mut Graphics2D, view_size: Vec2) {
        let gl = GridLayout::new(self.antbox.bounds, view_size);
        let mut gfx = GfxLayout::new(g, gl);

        gfx.draw(layers::Background);
        gfx.draw(&self.food);
        gfx.draw(layers::WireFrame);
    }
}

impl<R> UpdateInput<&mut R> for AnimationState
where
    R: rand::Rng,
{
    fn update_input(self, r: &mut R) -> Self {
        let AnimationState { antbox, food } = self;

        let antbox = antbox.into_next();
        let food = food.update_input((r, &antbox));

        AnimationState { antbox, food }
    }
}
