use antbox_state::GenParams;
use clap::Parser;

#[derive(Parser)]
pub struct Options {
    #[clap(flatten)]
    pub logopts: logging_options::StandardConsole,

    /// The RNG seed
    #[clap(long, default_value = "0")]
    pub seed: u64,

    #[clap(flatten)]
    pub genparams: GenParams,
}
