use antbox_geom::{BoundPoint, DirSet};
use derive_more::{IsVariant, TryInto};
use either::Either::{self, Left, Right};
use movestate::into::IntoNextWith;
use rand::distr::Distribution;

use crate::spotupdate::SpotUpdate;
use crate::{OptInto as _, Pheromone, Pheromones, SeedPod, State, SteppedUpon};

use self::AntMode::*;

/// The state of an ant
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ant {
    pub(crate) mode: AntMode,
    ph_here: Pheromones,
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
    pub(crate) fn new_from_ant_hole(mode: AntMode) -> Self {
        Ant {
            mode,
            ph_here: Pheromones::new(0, u8::MAX),
        }
    }

    /// The [Pheromones] underneath this ant
    pub fn pheromones_underneath(self) -> Pheromones {
        self.ph_here
    }

    pub(crate) fn stepped_on_empty(self, ph_new: Pheromones) -> Self {
        let ph_here = ph_new + (self.ph_here - ph_new).clamp();
        log::debug!("ph_new {ph_new:?} -> ph_here {ph_here:?}");
        Ant {
            mode: self.mode,
            ph_here,
        }
    }

    pub(crate) fn opt_with(self, pod: SeedPod) -> Option<Self> {
        match self.mode {
            Exploring | Hungry => Some(Ant {
                mode: WithFood(pod),
                ..self
            }),
            WithFood(_) => None,
        }
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
                    // If there's no adjacent food, follow pheromones
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

impl<'a, R> IntoNextWith<SpotUpdate<'a, R>> for Ant
where
    R: rand::Rng,
{
    type Next = (Either<Self, Pheromones>, Option<BoundPoint>);

    fn into_next_with(self, su: SpotUpdate<'a, R>) -> Self::Next {
        let dirs = self.mode.sense(su.state, su.pt);
        let dir = dirs.sample(su.rng).unwrap();
        let dst = su.pt + dir;

        if su.state.move_ant(self, dst) {
            (Right(self.ph_here), Some(dst))
        } else {
            (Left(self), None)
        }
    }
}

impl SteppedUpon for Ant {
    type NewState = Self;

    fn stepped_upon_by(self, other: Ant) -> Option<Self> {
        log::debug!("Bonk! {self:?}.stepped_upon_by({other:?})");
        None
    }
}
