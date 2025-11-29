mod antbox;
mod cliopts;
mod colors;
mod ticktimer;
mod window;

use clap::Parser as _;
use logging_options::Backend as _;
use speedy2d::error::BacktraceError;
use speedy2d::window::WindowCreationError;

pub type Result<T> = std::result::Result<T, BacktraceError<WindowCreationError>>;

pub use self::antbox::AntBox;
pub use self::cliopts::Options;
pub use self::ticktimer::TickTimer;
pub use self::window::AntBoxWindow;

pub fn run() -> Result<()> {
    let opts = Options::parse();
    env_logger::Logger::init_from_options(&opts.logopts);

    AntBoxWindow::new(0, 0.7).run()
}
