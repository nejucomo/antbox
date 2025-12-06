use antbox_state::GenParams;
use clap::Parser;

#[derive(Parser)]
pub struct Options {
    #[clap(flatten)]
    pub logopts: logging_options::StandardConsole,

    #[clap(flatten)]
    pub genparams: GenParams,
}
