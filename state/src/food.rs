use movestate::Transform;
use rand::distr::Distribution as _;

use crate::consts::{LIFE_CHANGE_DENOM, SEED_CHANGE_DENOM, WCOIN_POD_DISAPPEARS};
use crate::spotupdate::SpotUpdate;
use crate::{Ant, SteppedUpon};

/// Yum!
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Food {
    /// How many seeds are in this pod?
    pub seeds: u8,
    /// Is the pod fully developed?
    pub alive: bool,
}

impl Food {
    fn is_empty_pod(self) -> bool {
        !self.alive && self.seeds == 0
    }
}

impl<'a, R> Transform<SpotUpdate<'a, R>> for Food
where
    R: rand::Rng,
{
    type Next = Option<Self>;

    fn transform(self, su: SpotUpdate<'a, R>) -> Self::Next {
        let (target_life, target_nc) = su.state.growth_and_neighbors(su.pt);

        let delta = (target_nc as i8) - (self.seeds as i8);
        let dabs = delta.unsigned_abs() as u32;

        let newseeds = (self.seeds as i8)
            + if delta != 0 && su.rng.random_ratio(dabs, SEED_CHANGE_DENOM) {
                delta.signum()
            } else {
                0i8
            };

        let next = Food {
            seeds: newseeds as u8,
            alive: if self.alive == target_life {
                self.alive
            } else if delta == 0 && su.rng.random_ratio(dabs, LIFE_CHANGE_DENOM) {
                target_life
            } else {
                self.alive
            },
        };

        if next.is_empty_pod() && WCOIN_POD_DISAPPEARS.sample(su.rng) {
            None
        } else {
            Some(next)
        }
    }
}

impl SteppedUpon for Food {
    type NewState = Ant;

    fn stepped_upon_by(self, ant: Ant) -> Option<Ant> {
        use Ant::WithFood;

        match ant {
            // `ant` can only hold one: Bonk!
            WithFood(_) => None,
            // `ant` picks me up!
            _ => Some(WithFood(self)),
        }
    }
}
