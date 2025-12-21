use antbox_geom::BoundPoint;
use derive_more::{From, IsVariant, TryInto};
use movestate::Transform;
use rand::distr::Distribution as _;

use crate::consts::{PHEROMONE_SEED_POD_DIES, WCOIN_POD_APPEARS, WCOIN_POD_UPDATES};
use crate::spotupdate::SpotUpdate;
use crate::{Ant, AntHole, Objectish, Pheromones, SeedPod, SteppedUpon};

/// A [Spot] in the [State](crate::State)
#[derive(Copy, Clone, Debug, From, TryInto, IsVariant)]
pub enum Spot {
    /// Nothing is here except pheromones
    Empty(Pheromones),
    /// A [SeedPod]
    Food(SeedPod),
    /// An [Ant]
    Ant(Ant),
    /// An [AntHole]
    AntHole(AntHole),
}

impl Spot {
    pub(crate) fn pheromones(self) -> Pheromones {
        use Spot::*;

        match self {
            Empty(ph) => ph,
            Ant(ant) => ant.pheromones_underneath(),
            _ => Pheromones::default(),
        }
    }
}

impl Default for Spot {
    fn default() -> Self {
        Pheromones::default().into()
    }
}

impl<'a, R> Transform<SpotUpdate<'a, R>> for Spot
where
    R: rand::Rng,
{
    type Next = (Self, Option<BoundPoint>);

    fn transform(self, su: SpotUpdate<'a, R>) -> Self::Next {
        use Spot::*;

        match self {
            Empty(ph) => {
                if su.state.food_is_growing(su.pt)
                    && WCOIN_POD_UPDATES.sample(su.rng)
                    && WCOIN_POD_APPEARS.sample(su.rng)
                {
                    (Food(SeedPod::default()), None)
                } else {
                    (Empty(ph.transform(su.rng)), None)
                }
            }
            Food(pod) => {
                if let Some(pod) = pod.transform(su) {
                    (Food(pod), None)
                } else {
                    (Empty(Pheromones::new(PHEROMONE_SEED_POD_DIES, 0)), None)
                }
            }
            Ant(ant) => {
                let (antorph, optbp) = ant.transform(su);
                (antorph.either(Ant, Empty), optbp)
            }
            AntHole(ah) => {
                let (ah, optbp) = ah.transform(su);
                (AntHole(ah), optbp)
            }
        }
    }
}

impl Objectish for Spot {}

impl SteppedUpon for Spot {
    type NewState = Self;

    fn stepped_upon_by(self, ant: Ant) -> Option<Self> {
        use Spot::*;

        match self {
            Empty(ph) => Some(Ant(ant.stepped_on_empty(ph))),
            Food(pod) => pod.stepped_upon_by(ant).map(Ant),
            Ant(incumbent) => incumbent.stepped_upon_by(ant).map(Ant),
            AntHole(ah) => ah.stepped_upon_by(ant).map(AntHole),
        }
    }
}
