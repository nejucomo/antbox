use std::ops::DerefMut;

use antbox_state::{Pheromones, SeedPod, Spot};
use rand::Rng;

pub(super) fn enumerate_spot_render_states<R: rand::Rng>(rng: R) -> impl Iterator<Item = Spot> {
    RngInserter {
        rng,
        spot: EnumStates(Spot::default()),
    }
    .into_iterator()
}

struct RngInserter<R> {
    rng: R,
    optspot: Option<Spot>,
}

impl<R> TakeIntoNext<()> for RngInserter<R>
where
    R: DerefMut,
    R::Target: Rng,
{
    type Next = Option<(Self, Spot)>;

    fn take_into_next(self, (): ()) -> Self::Next {
        let RngInserter { mut rng, optspot } = self;

        if let Some(prevspot) = optspot {
            if let Some(nextspot) = prevspot.take_into_opt_self((EnumStates, rng.deref_mut())) {
                Some((
                    RngInserter {
                        rng,
                        optspot: Some(nextspot),
                    },
                    nextspot,
                ))
            } else {
                None
            }
        } else {
            let sp = Spot::default();
            Some((
                RngInserter {
                    rng,
                    optspot: Some(spot),
                },
                spot,
            ))
        }
    }
}

struct EnumStates;

impl<R: Rng> TakeIntoNext<R> for EnumStates<Spot> {
    type Next = Halting<State<Self>>;

    fn take_into_next(self, rng: R) -> Self::Next {
        use Spot::*;

        match self.0 {
            Empty(ph) => EnumStates(ph)
                .take_into_opt_self(rng)
                .map(Empty)
                .unwrap_or_else(|| Food(SeedPod::default())),
            Food(seed_pod) => todo!(),
            Ant(ant) => todo!(),
            AntHole(ant_hole) => todo!(),
        }
    }
}

impl<R: Rng> TakeIntoNext<R> for EnumStates<Pheromones> {
    type Next = Halting<State<Self>>;
}
