use antbox_geom::BoundPoint;
use movestate::UpdateInput;

use crate::consts::{
    LIFE_FORCE_ANT_RETURNS, LIFE_FORCE_FOOD_LIFE, LIFE_FORCE_FOOD_SEED, LIFE_FORCE_SPAWN_ANT,
};
use crate::{Ant, State, SteppedUpon};

/// An [AntHole] collects food for its lifeforce and uses that to spawn ants
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AntHole {
    lifeforce: usize,
}

impl<R> UpdateInput<(&mut R, &State, BoundPoint)> for AntHole
where
    R: rand::Rng,
{
    fn update_input(self, (rng, state, pt): (&mut R, &State, BoundPoint)) -> Self {
        if self.lifeforce > LIFE_FORCE_SPAWN_ANT {
            // BUG: We need a way to write to `state` to spawn ants
        }
    }
}

impl SteppedUpon for AntHole {
    type NewState = Self;

    fn stepped_upon_by(self, ant: Ant) -> Option<Self> {
        use Ant::*;

        Some(AntHole {
            lifeforce: self.lifeforce
                + LIFE_FORCE_ANT_RETURNS
                + match ant {
                    WithFood(food) => {
                        (food.seeds as usize) * LIFE_FORCE_FOOD_SEED
                            + if food.alive { LIFE_FORCE_FOOD_LIFE } else { 0 }
                    }
                    _ => 0,
                },
        })
    }
}

impl Default for AntHole {
    fn default() -> Self {
        Self { lifeforce: 30 }
    }
}
