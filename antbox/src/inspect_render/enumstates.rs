use std::cell::RefCell;
use std::rc::Rc;

use antbox_state::{Pheromones, SeedPod, Spot};
use movestate::next::{Halting, Stout};
use movestate::{IntoHaltingStout as _, TakeIntoNext};
use rand::Rng;

pub(super) fn enumerate_spot_render_states<R: Rng>(rng: R) -> impl Iterator<Item = Spot> {
    EnumSpots(Spot::default())
        .capture_clone(Rc::new(RefCell::new(rng)))
        .into_iterator()
}

struct EnumSpots(Spot);

impl<R: Rng> TakeIntoNext<Rc<RefCell<R>>> for EnumSpots {
    type Next = Halting<Stout<Self, Spot>>;

    fn take_into_next(self, rrr: Rc<RefCell<R>>) -> Self::Next {
        let mut bmut = rrr.borrow_mut();
        let rng = &mut *bmut;
        self.0
            .enum_next(rng)
            .map(|s: Spot| Stout::new(Self(s), s))
            .into()
    }
}

trait EnumInterestingValues: Sized {
    fn enum_next<R: Rng>(self, rng: &mut R) -> Option<Self>;
}

impl EnumInterestingValues for Spot {
    fn enum_next<R: Rng>(self, rng: &mut R) -> Option<Self> {
        use Spot::*;

        match self {
            Empty(x) => Some(
                x.enum_next(rng)
                    .map(Empty)
                    .unwrap_or_else(|| Food(SeedPod::default())),
            ),
            Food(x) => Some(
                x.enum_next(rng)
                    .map(Food)
                    .unwrap_or_else(|| Ant(Default::default())),
            ),
            Ant(_) => None,     // TODO
            AntHole(_) => None, // TODO
        }
    }
}

impl EnumInterestingValues for Pheromones {
    fn enum_next<R: Rng>(self, rng: &mut R) -> Option<Self> {
        let Pheromones { food, home } = self;

        let mut rr_above = |n: u8| n.saturating_add(rng.random_range(1..=n));

        // Diamond traversal:
        match (food, home) {
            (u8::MAX, u8::MAX) => None,
            (0, 0) => Some((1, 0)),
            (u8::MAX, 0) => Some((0, 1)),
            (0, u8::MAX) => Some((1, 1)),
            (f, 0) => Some((rr_above(f), 0)),
            (0, h) => Some((0, rr_above(h))),
            (f, h) => Some((rr_above(f), rr_above(h))),
        }
        .map(Pheromones::from)
    }
}

impl EnumInterestingValues for SeedPod {
    fn enum_next<R: Rng>(self, _: &mut R) -> Option<Self> {
        if self.seeds == 8 {
            if self.ripe {
                None
            } else {
                Some(SeedPod::new(0, true))
            }
        } else {
            Some(SeedPod::new(self.seeds + 1, self.ripe))
        }
    }
}
