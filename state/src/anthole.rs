use antbox_geom::{BoundPoint, Direction};
use movestate::Transform;
use rand::distr::Distribution as _;

use crate::consts::{
    LIFE_FORCE_ANT_RETURNS, LIFE_FORCE_FOOD_LIFE, LIFE_FORCE_FOOD_SEED, LIFE_FORCE_SPAWN_ANT,
    WCOIN_LIFE_FORCE_LOSS,
};
use crate::spotupdate::SpotUpdate;
use crate::{Ant, Pheromones, SteppedUpon};

/// An [AntHole] collects food for its lifeforce and uses that to spawn ants
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AntHole {
    /// Our lifeforce
    lifeforce: usize,
    /// Number of ants in the field
    ants: u32,
}

impl Default for AntHole {
    fn default() -> Self {
        Self {
            lifeforce: 15,
            ants: 0,
        }
    }
}

impl<'a, R> Transform<SpotUpdate<'a, R>> for AntHole
where
    R: rand::Rng,
{
    type Next = (AntHole, Option<BoundPoint>);

    fn transform(self, su: SpotUpdate<'a, R>) -> Self::Next {
        use crate::AntMode::{Exploring, Hungry};

        if self.lifeforce > LIFE_FORCE_SPAWN_ANT && su.rng.random_ratio(1, 1 + self.ants) {
            let newant = if self.lifeforce > 2 * LIFE_FORCE_SPAWN_ANT {
                Ant::new(Exploring, Pheromones::new(0, 201))
            } else {
                Ant::new(Hungry, Pheromones::new(0, 49))
            };

            let antpt = su.pt + su.rng.random::<Direction>();

            if su.state.move_ant(newant, antpt) {
                let newh = AntHole {
                    lifeforce: self.lifeforce - LIFE_FORCE_SPAWN_ANT,
                    ants: self.ants + 1,
                };
                log::info!("New ant {newant:?} at {antpt:?} from {newh:?}");
                (newh, Some(antpt))
            } else {
                log::debug!("Spawning failed for ant {newant:?} as {antpt:?}");
                (self, None)
            }
        } else if WCOIN_LIFE_FORCE_LOSS.sample(su.rng) {
            (
                AntHole {
                    lifeforce: self.lifeforce - 1,
                    ..self
                },
                None,
            )
        } else {
            (self, None)
        }
    }
}

impl SteppedUpon for AntHole {
    type NewState = Self;

    fn stepped_upon_by(self, ant: Ant) -> Option<Self> {
        let newh = AntHole {
            lifeforce: self.lifeforce
                + LIFE_FORCE_ANT_RETURNS
                + if let Some(pod) = ant.seed_pod() {
                    (pod.seeds as usize) * LIFE_FORCE_FOOD_SEED
                        + if pod.ripe { LIFE_FORCE_FOOD_LIFE } else { 0 }
                } else {
                    0
                },
            ants: self.ants - 1,
        };

        log::info!("Ant {ant:?} stepped on {newh:?}");

        Some(newh)
    }
}
