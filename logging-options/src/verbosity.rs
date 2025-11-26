use clap::Args;
use log::LevelFilter;

use crate::backend::LoggingOptions;

/// A mutually exclusive set of arguments to specify log verbosity
#[derive(Copy, Clone, Debug, Default, Args)]
#[group(multiple = false)]
pub struct Verbosity {
    /// Do not log any messages
    #[arg(short, long, help_heading = "Log Verbosity")]
    silent: bool,

    /// Log only errors
    #[arg(short, long, help_heading = "Log Verbosity")]
    quiet: bool,

    /// Log verbose info messages
    #[arg(short, long, help_heading = "Log Verbosity")]
    verbose: bool,

    /// Log debug messages
    #[arg(short, long, help_heading = "Log Verbosity")]
    debug: bool,

    /// Log highly verbose trace messages
    #[arg(short, long, help_heading = "Log Verbosity")]
    trace: bool,
}

impl LoggingOptions<env_logger::Logger> for Verbosity {
    fn configure(&self, mut builder: env_logger::Builder) -> env_logger::Builder {
        builder.filter_level((*self).into());
        builder
    }
}

impl From<Verbosity> for LevelFilter {
    fn from(verbosity: Verbosity) -> Self {
        use LevelFilter::*;

        let Verbosity {
            silent,
            quiet,
            verbose,
            debug,
            trace,
        } = verbosity;

        match (silent, quiet, verbose, debug, trace) {
            (true, false, false, false, false) => Off,
            (false, true, false, false, false) => Error,
            (false, false, false, false, false) => Warn,
            (false, false, true, false, false) => Info,
            (false, false, false, true, false) => Debug,
            (false, false, false, false, true) => Trace,
            _ => panic!(
                "`LogVerbosity` should not be instantiated with more than one true field: {verbosity:#?}"
            ),
        }
    }
}

impl From<LevelFilter> for Verbosity {
    fn from(level: LevelFilter) -> Self {
        use LevelFilter::*;

        let mut lv = Verbosity::default();
        let mut dummy_warn = false;

        let fieldptr = match level {
            Trace => &mut lv.trace,
            Debug => &mut lv.debug,
            Info => &mut lv.verbose,
            Warn => &mut dummy_warn,
            Error => &mut lv.quiet,
            Off => &mut lv.silent,
        };

        // Set only a single field (maybe the dummy):
        *fieldptr = true;

        lv
    }
}
