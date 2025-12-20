use antbox_geom::{BoundPoint, DirSet};
use derive_more::{IsVariant, TryInto};
use derive_new::new;
use movestate::Transform;
use rand::distr::Distribution;

use crate::spotupdate::SpotUpdate;
use crate::{Objectish as _, OptInto as _, Pheromone, Pheromones, SeedPod, State, SteppedUpon};

use self::AntMode::*;

/// The state of an ant
#[derive(Copy, Clone, Debug, Eq, PartialEq, new)]
pub struct Ant {
    pub(crate) mode: AntMode,
    ph: Pheromones,
}

/// An [Ant]'s mode
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryInto, IsVariant)]
pub enum AntMode {
    /// The ant is exploring
    Exploring,
    /// The ant is hungry
    Hungry,
    /// The ant has food
    WithFood(SeedPod),
}

impl Ant {
    pub(crate) fn opt_with(self, pod: SeedPod) -> Option<Self> {
        match self.mode {
            Exploring | Hungry => Some(Ant {
                mode: WithFood(pod),
                ..self
            }),
            WithFood(_) => None,
        }
    }

    pub(crate) fn pheromone_deposit(self) -> Pheromones {
        self.ph.deposit()
    }

    pub(crate) fn seed_pod(self) -> Option<SeedPod> {
        self.mode.opt_into()
    }
}

impl AntMode {
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
    type Next = (Option<Self>, Option<BoundPoint>);

    fn transform(self, su: SpotUpdate<'a, R>) -> Self::Next {
        let Ant { mode, ph } = self;
        let dirs = mode.sense(su.state, su.pt);
        let dir = dirs.sample(su.rng).unwrap();
        let dst = su.pt + dir;
        if su.state.move_ant(
            Ant {
                mode,
                ph: ph.decay(),
            },
            dst,
        ) {
            (None, Some(dst))
        } else {
            (Some(self), None)
        }
    }
}

impl SteppedUpon for Ant {
    type NewState = Self;

    fn stepped_upon_by(self, _: Ant) -> Option<Self> {
        // Watch where you're walking, buddy!
        None
    }
}
