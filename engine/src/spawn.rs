use antbox_cellauto::Evolvable as _;
use antbox_state::GenParams;

use crate::{Notifier, TickTimer};

/// Spawn the engine thread
pub fn spawn<N>(genparams: GenParams, notifier: N)
where
    N: Notifier + 'static,
{
    std::thread::spawn(move || {
        if let Err(err) = run_inner(genparams, notifier) {
            log::error!("{err}");
        }
    });
}

fn run_inner<N>(genparams: GenParams, notifier: N) -> Result<(), N::Error>
where
    N: Notifier,
{
    let mut state = genparams.generate_state();
    let mut tt = TickTimer::start(200); // TODO: allow application control

    loop {
        let next = state.evolve();
        let previous = std::mem::replace(&mut state, next);
        notifier.post(previous)?;
        tt.wait_for_tick();
    }
}
