use derive_new::new;
use movestate::Transform;

use self::Pheromone::*;

const DECAY_DENOMINATOR: u32 = 37;

/// The local levels of the [Pheromone]s
#[derive(Copy, Clone, Debug, Default, new)]
pub struct Pheromones {
    food: u8,
    home: u8,
}

/// A kind of [Ant](crate::Ant) pheromone
#[derive(Copy, Clone, Debug)]
pub enum Pheromone {
    /// Where to find food
    Food,
    /// Where to find home
    Home,
}

impl Pheromones {
    /// If there is no pheromone here
    pub fn is_empty(self) -> bool {
        self.food == 0 && self.home == 0
    }

    /// Get the local magnitude of the [Pheromone] type
    pub fn magnitude(self, ph: Pheromone) -> u8 {
        match ph {
            Food => self.food,
            Home => self.home,
        }
    }
}

impl<R> Transform<&mut R> for Pheromones
where
    R: rand::Rng,
{
    type Next = Self;

    fn transform(self, rng: &mut R) -> Self {
        if !self.is_empty() && rng.random_ratio(1, DECAY_DENOMINATOR) {
            Pheromones::new(self.food - 2.min(self.food), self.home - 1.min(self.home))
        } else {
            self
        }
    }
}
