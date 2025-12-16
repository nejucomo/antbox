use crate::Ant;

use self::Object::{AntHole, Food};

/// The type of [Object]s which can be in a [Spot](crate::Spot) in the [State](crate::State)
#[derive(Copy, Clone, Debug)]
pub enum Object {
    /// A food particle
    Food,
    /// An ant
    Ant(Ant),
    /// An ant hole
    AntHole,
}

/// Methods shared by [Object] and [Spot](crate::Spot)
pub trait Objectish: Sized + Copy {
    /// Is this food?
    fn is_food(self) -> bool;

    /// Is this an [Ant]?
    fn is_ant(self) -> bool {
        self.as_ant().is_some()
    }

    /// This as an [Ant]
    fn as_ant(self) -> Option<Ant>;

    /// This as an ant hole?
    fn is_ant_hole(self) -> bool;
}

impl Objectish for Object {
    fn is_food(self) -> bool {
        matches!(self, Food)
    }

    fn as_ant(self) -> Option<Ant> {
        match self {
            Object::Ant(a) => Some(a),
            _ => None,
        }
    }

    fn is_ant_hole(self) -> bool {
        matches!(self, AntHole)
    }
}
