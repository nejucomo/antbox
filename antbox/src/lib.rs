pub mod inspect_render;
pub mod options;
pub mod window;

use antbox_gameboard::GenParams;
use clap::Parser as _;
use logging_options::Backend as _;
use rand::SeedableRng as _;
use rand::rngs::StdRng;
use speedy2d::error::BacktraceError;
use speedy2d::window::WindowCreationError;

use crate::options::Command;

pub type Result<T> = std::result::Result<T, BacktraceError<WindowCreationError>>;

const TARGET_FRAME_RATE: f64 = 50.0;

/// A unit newtype for readability of framerate ticks(rather than `()`)
#[derive(Copy, Clone, Debug)]
struct Tick;

pub fn run() -> Result<()> {
    let opts = self::options::Options::parse();
    env_logger::Logger::init_from_options(&opts.logopts);

    log::debug!("Initializing RNG from seed {}.", opts.seed);
    let rng = StdRng::seed_from_u64(opts.seed);

    opts.cmd.unwrap_or_default().run(rng, opts.genparams)
}

impl Command {
    pub fn run<R>(self, rng: R, gp: GenParams) -> Result<()>
    where
        R: rand::Rng + 'static,
    {
        use Command::*;

        match self {
            Run => window::run(rng, gp),
            InspectRender => inspect_render::run(rng, gp),
        }
    }
}
