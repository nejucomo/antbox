use antbox_state::{Pheromones, Spot};
use movestate::TakeIntoNext;

pub(super) fn enumerate_spot_render_states<R: rand::Rng>(
    rng: &mut R,
) -> impl Iterator<Item = Spot> + '_ {
    ErsIterator {
        rng,
        optrse: Some(RseSpot::Empty(RsePheromones::Empty)),
    }
}

struct ErsIterator<'r, R> {
    rng: &'r mut R,
    optrse: Option<RseSpot>,
}

impl<'r, R: rand::Rng + 'static> Iterator for ErsIterator<'r, R> {
    type Item = Spot;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(rse) = self.optrse.take() {
            if let Some(next, st) = rse.take_into_next(self.rng) {
                self.optrse = Some(next);
                return Some(st);
            }
        }
        None
    }
}

enum RseSpot {
    Empty(RsePheromones),
}

enum RsePheromones {
    #[default]
    Empty,
}

impl<'r, R: rand::Rng + 'static> TakeIntoNext<&'r mut R> for RseSpot {
    type Next = Option<(Self, Spot)>;

    fn take_into_next(self, rng: &'r mut R) -> Self::Next {
        use RseSpot::*;

        match self {
            Empty(rseph) => todo!(),
        }
    }
}

impl<'r, R: rand::Rng + 'static> TakeIntoNext<&'r mut R> for RsePheromones {
    type Next = Option<(Self, Pheromones)>;

    fn take_into_next(self, rng: &'r mut R) -> Self::Next {
        use RsePheromones::*;

        match self {
            Empty => Some((
        }
    }
}
