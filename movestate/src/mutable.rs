//! Facilities for mutable state

mod next_output;
mod optext;
mod slot;
mod update;
mod update_as_take_into_starg;

pub use self::next_output::NextOutput;
pub use self::slot::Slot;
pub use self::update::Update;
pub use self::update_as_take_into_starg::UpdateAsTakeIntoStarg;
