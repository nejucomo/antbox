use antbox_grid::{DirSet, GridCoord};
use derive_more::{IsVariant, TryInto};
use either::Either::{self, Left, Right};
use mstate::TakeIntoNext;
use rand::distr::Distribution;

use crate::interesting::Interesting;
use crate::spotupdate::SpotUpdate;
use crate::{Field, OptInto as _, Pheromone, Pheromones, SeedPod, SteppedUpon};

use self::AntMode::*;

/// The state of an ant
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ant {
    /// The current behavioral mode
    pub mode: AntMode,
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
    /// Construct a new ant in the given `mode` as if from an [AntHole](crate::AntHole)
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

impl Default for Ant {
    fn default() -> Self {
        Ant {
            mode: Exploring,
            ph_here: Pheromones::default(),
        }
    }
}

impl AntMode {
    fn sense(self, state: &mut Field, pt: GridCoord) -> DirSet {
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

impl<'a, R> TakeIntoNext<SpotUpdate<'a, R>> for Ant
where
    R: rand::Rng,
{
    type Next = (Either<Self, Pheromones>, Option<GridCoord>);

    fn take_into_next(self, su: SpotUpdate<'a, R>) -> Self::Next {
        let dirs = self.mode.sense(su.field, su.pt);
        let dir = dirs.sample(su.rng).unwrap();
        let dst = su.pt + dir;

        if su.field.move_ant(self, dst) {
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

impl Interesting for Ant {
    fn first_interesting() -> Self {
        Ant {
            mode: Exploring,
            ph_here: Pheromones::default(),
        }
    }

    fn next_interesting<R: rand::Rng>(self, rng: &mut R) -> Option<Self> {
        self.mode
            .next_interesting(rng)
            .map(|mode| Ant { mode, ..self })
    }
}

impl Interesting for AntMode {
    fn first_interesting() -> Self {
        Self::Exploring
    }

    fn next_interesting<R: rand::Rng>(self, rng: &mut R) -> Option<Self> {
        match self {
            Exploring => Some(Hungry),
            Hungry => Some(WithFood(SeedPod::first_interesting())),
            WithFood(x) => x.next_interesting(rng).map(WithFood),
        }
    }
}
