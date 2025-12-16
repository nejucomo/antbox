use antbox_clife::ConwayCell;

use self::Object::*;

/// The type of [Object]s which can be in a [Spot](crate::Spot) in the [State](crate::State)
#[derive(Debug)]
pub enum Object {
    /// A food particle
    Food,
    /// An ant
    Ant,
    /// An ant hole
    AntHole,
}

impl ConwayCell for Object {
    fn is_alive(&self) -> bool {
        matches!(self, Food)
    }

    fn set_alive(&mut self, alive: bool) {
        if alive {
            *self = Food;
        }
    }
}
