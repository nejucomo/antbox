use antbox_geom::{BoundPoint, DirSet};
use movestate::UpdateInput;
use rand::Rng;
use rand::distr::Distribution;

use crate::{Food, Objectish as _, Pheromone, State, SteppedUpon};

use self::Ant::*;

/// The state of an ant
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Ant {
    /// The ant is exploring
    Exploring,
    /// The ant is hungry
    Hungry,
    /// The ant has food
    WithFood(Food),
}

impl Ant {
    /// Take a step
    pub fn sense_then_step<R>(self, state: &mut State, rng: &mut R, pt: BoundPoint)
    where
        R: Rng,
    {
        let dirs = self.sense(state, pt);
        let dir = dirs.sample(rng).unwrap();
        state.move_ant(self, pt, pt + dir);
    }

    fn sense(self, state: &mut State, pt: BoundPoint) -> DirSet {
        use Pheromone as Ph;

        match self {
            Exploring => state
                .pheromone_gradient(pt, Ph::Food, false)
                .intersect(state.pheromone_gradient(pt, Ph::Home, false)),
            Hungry => {
                let foodirs = state.directions_where(pt, |spot| spot.contains::<Food>());
                if foodirs.is_empty() {
                    // If there's no adjacent food, follow pheremones
                    state.pheromone_gradient(pt, Ph::Food, true)
                } else {
                    // otherwise get the food!
                    foodirs
                }
            }
            WithFood(_) => state.pheromone_gradient(pt, Ph::Home, true),
        }
    }
}

impl<R> UpdateInput<(&mut R, &State, BoundPoint)> for Ant
where
    R: rand::Rng,
{
    fn update_input(self, (rng, state, pt): (&mut R, &State, BoundPoint)) -> Self {
        // BUG: we need a way to mutate state, and then disappear if step succeeds
        self
    }
}

impl SteppedUpon for Ant {
    type NewState = Self;

    fn stepped_upon_by(self, _: Ant) -> Option<Self> {
        // Watch where you're walking, buddy!
        None
    }
}
