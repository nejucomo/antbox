use antbox_geom::BoundPoint;
use derive_more::{From, TryInto};
use mealy_machine::UpdateInput;

use crate::{Ant, AntHole, Food, Objectish, State, SteppedUpon};

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

impl<R> UpdateInput<(&mut R, &State, BoundPoint)> for Object
where
    R: rand::Rng,
{
    fn update_input(self, bundle: (&mut R, &State, BoundPoint)) -> Self {
        use Object::*;

        match self {
            Food(x) => Food(x.update_input(bundle)),
            Ant(x) => Ant(x.update_input(bundle)),
            AntHole(x) => AntHole(x.update_input(bundle)),
        }
    }
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
