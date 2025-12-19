use movestate::Transform;
use rand::distr::Distribution as _;

use crate::consts::{
    LIFE_FORCE_ANT_RETURNS, LIFE_FORCE_FOOD_LIFE, LIFE_FORCE_FOOD_SEED, LIFE_FORCE_SPAWN_ANT,
    WCOIN_LIFE_FORCE_LOSS,
};
use crate::spotupdate::SpotUpdate;
use crate::{Ant, SteppedUpon};

/// An [AntHole] collects food for its lifeforce and uses that to spawn ants
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AntHole {
    /// Our lifeforce
    lifeforce: usize,
    /// Number of ants in the field
    ants: u32,
}

impl<'a, R> Transform<SpotUpdate<'a, R>> for AntHole
where
    R: rand::Rng,
{
    type Next = AntHole;

    fn transform(self, su: SpotUpdate<'a, R>) -> Self::Next {
        if self.lifeforce > LIFE_FORCE_SPAWN_ANT && su.rng.random_ratio(1, 1 + self.ants) {
            let newant = if self.lifeforce > 2 * LIFE_FORCE_SPAWN_ANT {
                Ant::Exploring
            } else {
                Ant::Hungry
            };

            if su.state.move_ant(newant, su.pt + su.rng.random()).is_none() {
                AntHole {
                    lifeforce: self.lifeforce - LIFE_FORCE_SPAWN_ANT,
                    ants: self.ants + 1,
                }
            } else {
                self
            }
        } else if WCOIN_LIFE_FORCE_LOSS.sample(su.rng) {
            AntHole {
                lifeforce: self.lifeforce - 1,
                ..self
            }
        } else {
            self
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

            ants: self.ants - 1,
        })
    }
}

impl Default for AntHole {
    fn default() -> Self {
        Self {
            lifeforce: 30,
            ants: 0,
        }
    }
}
