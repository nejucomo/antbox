use antbox_cellauto::{ConwaysLife, GenGen, Generation, Ruleset as _};
use antbox_geom::Bounds;
use rand::SeedableRng as _;
use rand::distr::Distribution;
use rand::rngs::StdRng;
use speedy2d::window::{EventLoopSendError, UserEventSender};

use crate::TickTimer;

pub struct AntBox {
    foodgrid: Generation,
    /// Food generation
    fgen: usize,
    evsender: UserEventSender<Generation>,
}

impl AntBox {
    pub fn new<B>(
        rngseed: u64,
        cellprob: f64,
        bounds: B,
        evsender: UserEventSender<Generation>,
    ) -> Self
    where
        B: Into<Bounds>,
    {
        let mut rng = StdRng::seed_from_u64(rngseed);
        let foodgrid = GenGen::new(cellprob, bounds.into()).sample(&mut rng);

        AntBox {
            foodgrid,
            fgen: 0,
            evsender,
        }
    }

    pub fn run(self) {
        self.run_inner().unwrap();
    }

    fn run_inner(mut self) -> Result<(), EventLoopSendError> {
        let mut tt = TickTimer::default();

        log::debug!("sending initial foodgrid");
        self.evsender.send_event(self.foodgrid.clone())?;

        loop {
            tt.wait_for_tick();

            let fg = self.foodgrid;
            self.foodgrid = ConwaysLife.next_generation(&fg);
            assert!(self.foodgrid != fg);

            self.evsender.send_event(fg)?;
        }
    }
}

impl std::fmt::Debug for AntBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AntBox")
            .field("rng", &"..")
            .field("foodgrid", &self.foodgrid)
            .field("fgen", &self.fgen)
            .field("evsender", &"..")
            .finish()
    }
}
