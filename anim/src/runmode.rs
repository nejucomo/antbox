use derive_more::IsVariant;

use RunMode::*;

/// [Running] or [Paused]
#[derive(Copy, Clone, Debug, IsVariant)]
pub enum RunMode {
    #[allow(missing_docs)]
    Running,
    #[allow(missing_docs)]
    Paused,
}

impl RunMode {
    /// Toggle the mode
    pub fn toggle(&mut self) {
        *self = match self {
            Running => Paused,
            Paused => Running,
        };
        log::info!("{self:?}");
    }
}
