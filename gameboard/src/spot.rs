use antbox_grid::GridCoord;
use derive_more::{From, IsVariant, TryInto};
use mstate::MStateIn;
use rand::distr::Distribution as _;

use crate::consts::{PHEROMONE_SEED_POD_DIES, WCOIN_POD_APPEARS, WCOIN_POD_UPDATES};
use crate::interesting::Interesting;
use crate::spotupdate::SpotUpdate;
use crate::{Ant, AntHole, OptInto, Pheromones, SeedPod, SteppedUpon};

/// A [Spot] in the [BoardState](crate::BoardState)
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
    /// Iterate over "interesting" [Spot] values; used for render inspection, for example
    pub fn interesting_values<R: rand::Rng>(rng: &mut R) -> impl Iterator<Item = Spot> + '_ {
        pub struct InterestingSpots<'r, R: rand::Rng>(Option<(&'r mut R, Spot)>);

        impl<'r, R: rand::Rng> Iterator for InterestingSpots<'r, R> {
            type Item = Spot;

            fn next(&mut self) -> Option<Self::Item> {
                self.0.take().map(|(rng, spot)| {
                    self.0 = spot.next_interesting(rng).map(|s| (rng, s));
                    spot
                })
            }
        }

        InterestingSpots(Some((rng, Spot::first_interesting())))
    }

    pub(crate) fn contains<T>(self) -> bool
    where
        Self: OptInto<T>,
    {
        self.opt_into().is_some()
    }

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

impl<'a, R> MStateIn<SpotUpdate<'a, R>> for Spot
where
    R: rand::Rng,
{
    type Next = (Self, Option<GridCoord>);

    fn into_with(self, su: SpotUpdate<'a, R>) -> Self::Next {
        use Spot::*;

        match self {
            Empty(ph) => {
                if su.field.food_is_growing(su.pt)
                    && WCOIN_POD_UPDATES.sample(su.rng)
                    && WCOIN_POD_APPEARS.sample(su.rng)
                {
                    (Food(SeedPod::default()), None)
                } else {
                    (Empty(ph.into_with(su.rng)), None)
                }
            }
            Food(pod) => {
                if let Some(pod) = pod.into_with(su) {
                    (Food(pod), None)
                } else {
                    (Empty(Pheromones::new(PHEROMONE_SEED_POD_DIES, 0)), None)
                }
            }
            Ant(ant) => {
                let (antorph, optbp) = ant.into_with(su);
                (antorph.either(Ant, Empty), optbp)
            }
            AntHole(ah) => {
                let (ah, optbp) = ah.into_with(su);
                (AntHole(ah), optbp)
            }
        }
    }
}

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

impl Interesting for Spot {
    fn first_interesting() -> Self {
        Self::Empty(Pheromones::first_interesting())
    }

    fn next_interesting<R: rand::Rng>(self, rng: &mut R) -> Option<Self> {
        fn subnext<R, A, B>(rng: &mut R, a: A) -> Option<Spot>
        where
            R: rand::Rng,
            A: Interesting,
            B: Interesting,
            Spot: From<A> + From<B>,
        {
            Some(
                a.next_interesting(rng)
                    .map(Spot::from)
                    .unwrap_or_else(|| Spot::from(B::first_interesting())),
            )
        }

        match self {
            Spot::Empty(x) => subnext::<_, _, SeedPod>(rng, x),
            Spot::Food(x) => subnext::<_, _, Ant>(rng, x),
            Spot::Ant(x) => subnext::<_, _, AntHole>(rng, x),
            Spot::AntHole(x) => x.next_interesting(rng).map(Spot::from),
        }
    }
}
