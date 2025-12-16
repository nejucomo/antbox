use antbox_geom::BoundPoint;
use rand::Rng;
use rand::distr::Distribution;

use crate::{Objectish as _, Pheromone, State};

/// The state of an ant
#[derive(Copy, Clone, Debug)]
pub enum Ant {
    /// The ant is exploring
    Exploring,
    /// The ant is hungry
    Hungry,
    /// The ant has food
    WithFood,
}

impl Ant {
    /// Take a step
    pub fn step<R>(self, state: &mut State, rng: &mut R, pt: BoundPoint)
    where
        R: Rng,
    {
        use Ant::*;
        use Pheromone::*;

        let dirs = match self {
            Exploring => state
                .pheromone_gradient(pt, Food, false)
                .intersect(state.pheromone_gradient(pt, Home, false)),
            Hungry => {
                let foodirs = state.directions_where(pt, |spot| spot.is_food());
                if foodirs.is_empty() {
                    // If there's no adjacent food, follow pheremones
                    state.pheromone_gradient(pt, Food, true)
                } else {
                    // otherwise get the food!
                    foodirs
                }
            }
            WithFood => state.pheromone_gradient(pt, Home, true),
        };

        let dir = dirs.sample(rng).unwrap();

        todo!("{dir:?}")
    }
}
