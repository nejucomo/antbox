use antbox_geom::BoundPoint;
use derive_new::new;
use mealy_machine::UpdateInput;
use rand::distr::Distribution as _;

use crate::consts::{WCOIN_POD_APPEARS, WCOIN_POD_DISAPPEARS};
use crate::{
    Ant, AntHole, Food, Object, Objectish, OptInto, Pheromone, Pheromones, State, SteppedUpon,
};

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

    /// The magnitude of the [Pheromone] at this [Spot]
    pub fn pheromone_magnitude(self, ph: Pheromone) -> u8 {
        self.pheromones.magnitude(ph)
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

impl<R> UpdateInput<(&mut R, &State, BoundPoint)> for Spot
where
    R: rand::Rng,
{
    fn update_input(self, (rng, state, pt): (&mut R, &State, BoundPoint)) -> Self {
        let pheromones = self.pheromones.update_input(rng);
        let fig = state.food_is_growing(pt);

        let obj = if let Some(prevobj) = self.obj {
            if !fig
                && prevobj
                    .opt_into()
                    .map(|f: Food| f.is_empty_pod())
                    .unwrap_or(false)
                && WCOIN_POD_DISAPPEARS.sample(rng)
            {
                None
            } else {
                Some(prevobj.update_input((rng, state, pt)))
            }
        } else if fig && WCOIN_POD_APPEARS.sample(rng) {
            Some(Food::default().into())
        } else {
            None
        };

        Spot { obj, pheromones }
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
            ..self
        })
    }
}

impl OptInto<Food> for Spot {
    fn opt_into(self) -> Option<Food> {
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
