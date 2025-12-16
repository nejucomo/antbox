use antbox_state::{GenParams, State as AntboxState};
use mealy_machine::toolkit::Cycler;
use mealy_machine::{IntoNext, UpdateInput};
use speedy2d::Graphics2D;
use speedy2d::dimen::Vec2;

use crate::{GfxLayout, GridLayout, TICKS_PER_CONWAY, layers};

/// Encapsulate a [AntboxState] with extra animation-specific state
#[derive(Debug)]
pub struct AnimationState {
    antbox: Cycler<AntboxState>,
    foodeco: layers::FoodDecoration,
}

impl AnimationState {
    /// Initialize
    pub fn new<R: rand::Rng>(rng: &mut R, gp: GenParams) -> Self {
        let antbox = gp.generate_state(rng);
        let bounds = antbox.bounds();
        AnimationState {
            antbox: Cycler::new(antbox, TICKS_PER_CONWAY),
            foodeco: layers::FoodDecoration::from(bounds),
        }
    }

    /// Draw `self` onto `gfx`
    pub fn draw(&self, g: &mut Graphics2D, view_size: Vec2) {
        let gl = GridLayout::new(self.antbox.bounds(), view_size);
        let mut gfx = GfxLayout::new(g, gl);

        gfx.draw(layers::Background);
        gfx.draw(layers::WireFrame);
        gfx.draw(&*self.antbox);
        gfx.draw(&self.foodeco);
    }
}

impl<R> UpdateInput<&mut R> for AnimationState
where
    R: rand::Rng,
{
    fn update_input(self, r: &mut R) -> Self {
        let AnimationState {
            antbox,
            foodeco: food,
        } = self;

        let antbox = antbox.into_next();
        let food = food.update_input((r, &antbox));

        AnimationState {
            antbox,
            foodeco: food,
        }
    }
}
