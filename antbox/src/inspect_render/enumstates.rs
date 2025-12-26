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

trait WrapperTrait: Sized {
    fn enum_next<R: Rng>(self, rng: &mut R) -> Option<Self>;
}

impl WrapperTrait for Spot {
    fn enum_next<R: Rng>(self, rng: &mut R) -> Option<Self> {
        use Spot::*;

        match self {
            Empty(x) => Some(
                x.enum_next(rng)
                    .map(Empty)
                    .unwrap_or_else(|| Food(SeedPod::default())),
            ),
            Food(_) => todo!(),
            Ant(_) => todo!(),
            AntHole(_) => todo!(),
        }
    }
}

impl WrapperTrait for Pheromones {
    fn enum_next<R: Rng>(self, _rng: &mut R) -> Option<Self> {
        todo!()
    }
}
