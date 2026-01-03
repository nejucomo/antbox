use derive_more::From;
use derive_new::new;
use mstate::TakeIntoNext;
use rand::Rng;
use rand::distr::Distribution as _;

use crate::consts::{
    LIFE_CHANGE_DENOM, SEED_CHANGE_DENOM, WCOIN_POD_DISAPPEARS, WCOIN_POD_UPDATES,
};
use crate::interesting::Interesting;
use crate::spotupdate::SpotUpdate;
use crate::{Ant, SteppedUpon};

/// Yum!
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, From, new)]
pub struct SeedPod {
    /// How many seeds are in this pod?
    pub seeds: u8,
    /// Is the pod fully developed?
    pub ripe: bool,
}

impl SeedPod {
    fn is_empty_pod(self) -> bool {
        !self.ripe && self.seeds == 0
    }
}

struct DoUpdate;

impl<'a, R> TakeIntoNext<SpotUpdate<'a, R>> for SeedPod
where
    R: rand::Rng,
{
    type Next = Option<Self>;

    fn take_into_next(self, su: SpotUpdate<'a, R>) -> Self::Next {
        if WCOIN_POD_UPDATES.sample(su.rng) {
            self.take_into_next((DoUpdate, su))
        } else {
            Some(self)
        }
    }
}

impl<'a, R> TakeIntoNext<(DoUpdate, SpotUpdate<'a, R>)> for SeedPod
where
    R: rand::Rng,
{
    type Next = Option<Self>;

    fn take_into_next(self, (_, su): (DoUpdate, SpotUpdate<'a, R>)) -> Self::Next {
        let (target_life, target_nc) = su.field.growth_and_neighbors(su.pt);

        let delta = (target_nc as i8) - (self.seeds as i8);
        let dabs = delta.unsigned_abs() as u32;

        let newseeds = (self.seeds as i8)
            + if delta != 0 && su.rng.random_ratio(dabs, SEED_CHANGE_DENOM) {
                delta.signum()
            } else {
                0i8
            };

        let next = SeedPod {
            seeds: newseeds as u8,
            ripe: if self.ripe == target_life {
                self.ripe
            } else if delta == 0 && su.rng.random_ratio(dabs, LIFE_CHANGE_DENOM) {
                target_life
            } else {
                self.ripe
            },
        };

        if next.is_empty_pod() && WCOIN_POD_DISAPPEARS.sample(su.rng) {
            None
        } else {
            Some(next)
        }
    }
}

impl SteppedUpon for SeedPod {
    type NewState = Ant;

    fn stepped_upon_by(self, ant: Ant) -> Option<Ant> {
        ant.opt_with(self)
    }
}

impl Interesting for SeedPod {
    fn first_interesting() -> Self {
        SeedPod::default()
    }

    fn next_interesting<R: Rng>(self, _: &mut R) -> Option<Self> {
        if self.seeds == 8 {
            if self.ripe {
                None
            } else {
                Some(SeedPod::new(0, true))
            }
        } else {
            assert!(self.seeds < 8);
            Some(SeedPod::new(self.seeds + 1, self.ripe))
        }
    }
}
