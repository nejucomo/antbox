use antbox_gameboard::GenParams;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Options {
    #[clap(flatten)]
    pub logopts: logging_options::StandardConsole,

    /// The RNG seed
    #[clap(long, default_value = "0")]
    pub seed: u64,

    #[clap(flatten)]
    pub genparams: GenParams,

    #[clap(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(Subcommand, Default)]
pub enum Command {
    #[default]
    Run,

    Inspector,
}
