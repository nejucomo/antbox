use derive_more::{From, TryInto};

use crate::{Ant, AntHole, Food, Objectish, SteppedUpon};

/// The type of [Object]s which can be in a [Spot](crate::Spot) in the [State](crate::State)
#[derive(Copy, Clone, Debug, From, Eq, PartialEq, TryInto)]
pub enum Object {
    /// A food particle
    Food(Food),
    /// An ant
    Ant(Ant),
    /// An ant hole
    AntHole(AntHole),
}

impl Objectish for Object {}

impl SteppedUpon for Object {
    type NewState = Self;

    fn stepped_upon_by(self, ant: Ant) -> Option<Self> {
        use Object::*;

        match self {
            Food(food) => food.stepped_upon_by(ant).map(Self::from),
            Ant(ant) => ant.stepped_upon_by(ant).map(Self::from),
            AntHole(h) => h.stepped_upon_by(ant).map(Self::from),
        }
    }
}
