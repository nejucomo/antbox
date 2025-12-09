mod anim;
mod cliopts;
mod colors;
mod window;

use clap::Parser as _;
use logging_options::Backend as _;
use speedy2d::error::BacktraceError;
use speedy2d::window::WindowCreationError;

pub type Result<T> = std::result::Result<T, BacktraceError<WindowCreationError>>;

/// A unit newtype for readability of framerate ticks(rather than `()`)
#[derive(Copy, Clone, Debug)]
struct Tick;

pub use self::cliopts::Options;
pub use self::window::AntBoxWindow;

pub fn run() -> Result<()> {
    let opts = Options::parse();
    env_logger::Logger::init_from_options(&opts.logopts);

    AntBoxWindow::run(opts.genparams)
}
