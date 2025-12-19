use antbox_geom::BoundPoint;
use movestate::UpdateInput;

use crate::consts::{LIFE_CHANGE_DENOM, SEED_CHANGE_DENOM};
use crate::{Ant, State, SteppedUpon};

/// Yum!
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Food {
    /// How many seeds are in this pod?
    pub seeds: u8,
    /// Is the pod fully developed?
    pub alive: bool,
}

impl Food {
    pub fn is_empty_pod(self) -> bool {
        !self.alive && self.seeds == 0
    }
}

impl<R> UpdateInput<(&mut R, &State, BoundPoint)> for Food
where
    R: rand::Rng,
{
    fn update_input(self, (rng, state, pt): (&mut R, &State, BoundPoint)) -> Self {
        let (target_life, target_nc) = state.life_and_neighbors(pt);

        let delta = (target_nc as i8) - (self.seeds as i8);
        let dabs = delta.abs() as u32;

        let newseeds = (self.seeds as i8)
            + if delta != 0 && rng.random_ratio(dabs, SEED_CHANGE_DENOM) {
                delta.signum()
            } else {
                0i8
            };

        Food {
            seeds: newseeds as u8,
            alive: if self.alive == target_life {
                self.alive
            } else if delta == 0 && rng.random_ratio(dabs, LIFE_CHANGE_DENOM) {
                target_life
            } else {
                self.alive
            },
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
