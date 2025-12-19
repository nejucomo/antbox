use antbox_clife::ConwayCell;
use derive_new::new;

use crate::{Ant, AntHole, Food, Object, Objectish, OptInto, Pheromone, Pheromones, SteppedUpon};

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

impl ConwayCell for Spot {
    fn is_alive(&self) -> bool {
        self.contains::<Food>()
    }

    fn set_alive(&mut self, alive: bool) {
        if alive && self.is_empty() {
            // We only set food life for empty spots; if it already contains food, this is a no-op, but if it contains an ant or anthole, those aren't overwritten.
            self.obj = Some(Food.into());
        } else if !alive && self.is_alive() {
            // It was `Food` so set it to nothing:
            self.obj = None;
        }
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
