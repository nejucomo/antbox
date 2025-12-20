use antbox_geom::BoundPoint;
use derive_new::new;
use movestate::{Transform, Update as _};
use rand::distr::Distribution as _;

use crate::consts::{WCOIN_POD_APPEARS, WCOIN_POD_UPDATES};
use crate::spotupdate::SpotUpdate;
use crate::{Ant, AntHole, Object, Objectish, OptInto, Pheromones, SeedPod, SteppedUpon};

/// Every [Spot] in the [State](crate::State) can contain up to one [Object]
#[derive(Copy, Clone, Debug, Default, new)]
pub struct Spot {
    obj: Option<Object>,
    #[new(default)]
    pheromones: Pheromones,
}

impl Spot {
    /// If this spot is unoccupied
    pub fn is_empty(self) -> bool {
        self.obj.is_none()
    }

    /// The [Object] in this [Spot]
    pub fn object(self) -> Option<Object> {
        self.obj
    }

    /// The [Pheromones] in this [Spot]
    pub fn pheromones(self) -> Pheromones {
        self.pheromones
    }

    /// Take the [Object]
    pub fn take_object(&mut self) -> Option<Object> {
        self.obj.take()
    }
}

impl<T> From<T> for Spot
where
    Object: From<T>,
{
    fn from(obj: T) -> Self {
        Self::new(Some(Object::from(obj)))
    }
}

impl<'a, R> Transform<SpotUpdate<'a, R>> for Spot
where
    R: rand::Rng,
{
    type Next = (Self, Option<BoundPoint>);

    fn transform(self, su: SpotUpdate<'a, R>) -> Self::Next {
        let pheromones = self.pheromones.update(su.rng);
        let fig = su.state.food_is_growing(su.pt);

        let (obj, stepdst) = if let Some(prevobj) = self.obj {
            prevobj.transform(su)
        } else if fig && WCOIN_POD_UPDATES.sample(su.rng) && WCOIN_POD_APPEARS.sample(su.rng) {
            (Some(SeedPod::default().into()), None)
        } else {
            (None, None)
        };

        (Spot { obj, pheromones }, stepdst)
    }
}

impl Objectish for Spot {}

impl SteppedUpon for Spot {
    type NewState = Self;

    fn stepped_upon_by(self, ant: Ant) -> Option<Self> {
        match self.obj {
            Some(obj) => obj.stepped_upon_by(ant),
            None => Some(ant.into()),
        }
        .map(|obj| Spot {
            obj: Some(obj),
            pheromones: self.pheromones
                + obj
                    .opt_into()
                    .map(|ant: Ant| ant.pheromone_deposit())
                    .unwrap_or_default(),
        })
    }
}

impl OptInto<SeedPod> for Spot {
    fn opt_into(self) -> Option<SeedPod> {
        self.obj.and_then(|obj| obj.opt_into())
    }
}

impl OptInto<Ant> for Spot {
    fn opt_into(self) -> Option<Ant> {
        self.obj.and_then(|obj| obj.opt_into())
    }
}

impl OptInto<AntHole> for Spot {
    fn opt_into(self) -> Option<AntHole> {
        self.obj.and_then(|obj| obj.opt_into())
    }
}
