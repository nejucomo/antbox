use antbox_clife::ConwayCell;

use crate::{Ant, Object, Objectish, Pheromone, Pheromones};

/// Every [Spot] in the [State](crate::State) can contain up to one [Object]
#[derive(Copy, Clone, Debug, Default)]
pub struct Spot {
    obj: Option<Object>,
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

    fn object_val<F, T>(self, f: F) -> T
    where
        F: FnOnce(Object) -> T,
        T: Default,
    {
        self.obj.map(f).unwrap_or_default()
    }
}

impl Objectish for Spot {
    fn is_food(self) -> bool {
        self.object_val(Object::is_food)
    }

    fn as_ant(self) -> Option<Ant> {
        self.obj.and_then(|obj| obj.as_ant())
    }

    fn is_ant_hole(self) -> bool {
        self.object_val(Object::is_ant_hole)
    }

    fn stepped_upon(&mut self, ant: Ant) -> bool {
        match self.obj.as_mut() {
            Some(obj) => obj.stepped_upon(ant),
            None => {
                self.obj = Some(ant.into());
                true
            }
        }
    }
}

impl ConwayCell for Spot {
    fn is_alive(&self) -> bool {
        self.is_food()
    }

    fn set_alive(&mut self, alive: bool) {
        if alive && self.is_empty() {
            // We only set food life for empty spots; if it already contains food, this is a no-op, but if it contains an ant or anthole, those aren't overwritten.
            self.obj = Some(Object::Food);
        } else if !alive && self.is_alive() {
            // It was `Food` so set it to nothing:
            self.obj = None;
        }
    }
}
