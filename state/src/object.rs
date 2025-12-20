use antbox_geom::BoundPoint;
use derive_more::{From, TryInto};
use movestate::{OptUpdate as _, Transform};

use crate::spotupdate::SpotUpdate;
use crate::{Ant, AntHole, Objectish, SeedPod, SteppedUpon};

/// The type of [Object]s which can be in a [Spot](crate::Spot) in the [State](crate::State)
#[derive(Copy, Clone, Debug, From, Eq, PartialEq, TryInto)]
pub enum Object {
    /// A food particle
    Food(SeedPod),
    /// An ant
    Ant(Ant),
    /// An ant hole
    AntHole(AntHole),
}

impl<'a, R> Transform<SpotUpdate<'a, R>> for Object
where
    R: rand::Rng,
{
    type Next = (Option<Object>, Option<BoundPoint>);

    fn transform(self, su: SpotUpdate<'a, R>) -> Self::Next {
        use Object::*;

        match self {
            Food(x) => (x.opt_update(su).map(Food), None),
            Ant(ant) => {
                let (optant, optdst) = ant.transform(su);
                (optant.map(Ant), optdst)
            }
            AntHole(ah) => {
                let (ah, optdst) = ah.transform(su);
                (Some(AntHole(ah)), optdst)
            }
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
