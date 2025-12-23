//! Facilities for mutable state

mod next_output;
mod optext;
mod slot;
mod update;

pub use self::next_output::NextOutput;
pub use self::slot::Slot;
pub use self::update::Update;
