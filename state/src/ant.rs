use antbox_geom::{BoundPoint, DirSet};
use movestate::Transform;
use rand::distr::Distribution;

use crate::spotupdate::SpotUpdate;
use crate::{SeedPod, Objectish as _, Pheromone, State, SteppedUpon};

use self::Ant::*;

/// The state of an ant
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Ant {
    /// The ant is exploring
    Exploring,
    /// The ant is hungry
    Hungry,
    /// The ant has food
    WithFood(SeedPod),
}

impl Ant {
    fn sense(self, state: &mut State, pt: BoundPoint) -> DirSet {
        use Pheromone as Ph;

        match self {
            Exploring => state
                .pheromone_gradient(pt, Ph::Food, false)
                .intersect(state.pheromone_gradient(pt, Ph::Home, false)),
            Hungry => {
                let foodirs = state.directions_where(pt, |spot| spot.contains::<SeedPod>());
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

impl<'a, R> Transform<SpotUpdate<'a, R>> for Ant
where
    R: rand::Rng,
{
    type Next = Option<Self>;

    fn transform(self, su: SpotUpdate<'a, R>) -> Self::Next {
        let dirs = self.sense(su.state, su.pt);
        let dir = dirs.sample(su.rng).unwrap();
        su.state.move_ant(self, su.pt + dir)
    }
}

impl SteppedUpon for Ant {
    type NewState = Self;

    fn stepped_upon_by(self, _: Ant) -> Option<Self> {
        // Watch where you're walking, buddy!
        None
    }
}
