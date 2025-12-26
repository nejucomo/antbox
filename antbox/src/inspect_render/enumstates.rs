use std::cell::RefCell;
use std::rc::Rc;

use antbox_state::{SeedPod, Spot};
use movestate::next::{Halting, Stout};
use movestate::{IntoHaltingStout as _, TakeIntoNext};
use rand::Rng;

pub(super) fn enumerate_spot_render_states<R: Rng>(rng: R) -> impl Iterator<Item = Spot> {
    EnumStates(Spot::default())
        .capture_clone(Rc::new(RefCell::new(rng)))
        .into_iterator()
}

struct EnumStates<T>(T);

impl<R: Rng> TakeIntoNext<Rc<RefCell<R>>> for EnumStates<Spot> {
    type Next = Halting<Stout<Self, Spot>>;

    fn take_into_next(self, rrr: Rc<RefCell<R>>) -> Self::Next {
        use Spot::*;

        match self.0 {
            Empty(ph) => EnumStates(ph)
                .take_into_opt_self(rrr)
                .map(Empty)
                .unwrap_or_else(|| Food(SeedPod::default())),
            Food(seed_pod) => todo!(),
            Ant(ant) => todo!(),
            AntHole(ant_hole) => todo!(),
        }
    }
}
