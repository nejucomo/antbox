use clap::Args;

use crate::Verbosity;
use crate::backend::LoggingOptions;

/// A bundle of [clap::Args] for console applications, allowing logging to stderr or a file
#[derive(Args)]
pub struct StandardConsole {
    #[clap(flatten)]
    verbosity: Verbosity,
}

impl LoggingOptions<env_logger::Logger> for StandardConsole {
    fn configure(&self, builder: env_logger::Builder) -> env_logger::Builder {
        self.verbosity.configure(builder)
    }
}
