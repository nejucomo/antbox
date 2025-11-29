use antbox_geom::Bounds;
use cellauto::{ConwaysLife, GenGen, Ruleset as _};
use rand::SeedableRng as _;
use rand::distr::Distribution;
use rand::rngs::StdRng;

use crate::{Notifier, TickTimer};

/// Spawn the engine thread
pub fn spawn<B, N>(rngseed: u64, cellprob: f64, bounds: B, notifier: N)
where
    B: Into<Bounds>,
    N: Notifier + 'static,
{
    let bounds = bounds.into();
    std::thread::spawn(move || run_thread(rngseed, cellprob, bounds, notifier));
}

fn run_thread<N>(rngseed: u64, cellprob: f64, bounds: Bounds, notifier: N)
where
    N: Notifier,
{
    if let Err(err) = run_inner(rngseed, cellprob, bounds, notifier) {
        log::error!("{err}");
    }
}

fn run_inner<N>(rngseed: u64, cellprob: f64, bounds: Bounds, notifier: N) -> Result<(), N::Error>
where
    N: Notifier,
{
    let mut rng = StdRng::seed_from_u64(rngseed);
    let mut gencnt = 0;
    let mut foodgrid = GenGen::new(cellprob, bounds).sample(&mut rng);
    let mut tt = TickTimer::start(200); // TODO: allow application control

    log::debug!("sending initial foodgrid");

    loop {
        let nextfg = ConwaysLife.next_generation(&foodgrid);
        let prevfg = std::mem::replace(&mut foodgrid, nextfg);

        notifier.post_new_food_generation(gencnt, prevfg)?;
        gencnt += 1;

        tt.wait_for_tick();
    }
}
