use derive_more::From;

use crate::Ant;

use self::Object::{AntHole, Food};

/// The type of [Object]s which can be in a [Spot](crate::Spot) in the [State](crate::State)
#[derive(Copy, Clone, Debug, From, Eq, PartialEq)]
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

    /// Modify this object based on `ant` attempting to step on it
    ///
    /// # Return
    ///
    /// Return if `ant` successfully stepped here.
    fn stepped_upon(&mut self, ant: Ant) -> bool;
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

    fn stepped_upon(&mut self, _: Ant) -> bool {
        let ons = match self {
            Food => Some(Ant::WithFood.into()),
            AntHole => todo!("step on anthole"),
            _ => None,
        };

        if let Some(nextself) = ons {
            *self = nextself;
        }
        ons.is_some()
    }
}
