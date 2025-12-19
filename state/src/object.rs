use derive_more::{From, TryInto};
use movestate::{OptUpdate as _, Transform, Update as _};

use crate::spotupdate::SpotUpdate;
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

impl<'a, R> Transform<SpotUpdate<'a, R>> for Object
where
    R: rand::Rng,
{
    type Next = Option<Object>;

    fn transform(self, su: SpotUpdate<'a, R>) -> Self::Next {
        use Object::*;

        match self {
            Food(x) => x.opt_update(su).map(Food),
            Ant(x) => x.opt_update(su).map(Ant),
            AntHole(x) => Some(AntHole(x.update(su))),
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
