use antbox_state::{GenParams, State as AntboxState};
use mealy_machine::{IntoNext, UpdateInput as _};
use rand::rngs::StdRng;
use speedy2d::Graphics2D;
use speedy2d::dimen::Vec2;

use crate::{GfxLayout, GridLayout, TICKS_PER_CONWAY, UpdateCycler, layers};

/// Encapsulate a [AntboxState] with extra animation-specific state
#[derive(Debug)]
pub struct AnimationState {
    rng: StdRng,
    antbox: UpdateCycler<AntboxState>,
    food: layers::Food,
}

impl AnimationState {
    /// Initialize
    pub fn new(gp: GenParams) -> Self {
        let (rng, antbox) = gp.generate_state();
        let bounds = antbox.bounds;
        AnimationState {
            rng,
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

impl IntoNext for AnimationState {
    fn into_next(self) -> Self {
        // TODO: Make the state stack `Update<&mut StdRng>`
        let AnimationState {
            mut rng,
            antbox,
            food,
        } = self;

        let antbox = antbox.into_next();
        let food = food.update_input((&mut rng, &antbox));

        AnimationState { rng, antbox, food }
    }
}
