use crate::{Result, Tick};

pub fn run<R>(rng: R, gp: GenParams) -> Result<()>
where
    R: rand::Rng + 'static,
{
    run_handler::<RinHandler<R>, R>(rng, gp)
}

#[derive(Dbg)]
struct WinHandler<R>
where
    R: rand::Rng,
{
