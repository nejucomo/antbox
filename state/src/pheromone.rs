use derive_new::new;
use mealy_machine::UpdateInput;

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
    /// Get the local magnitude of the [Pheromone] type
    pub fn magnitude(self, ph: Pheromone) -> u8 {
        match ph {
            Food => self.food,
            Home => self.home,
        }
    }
}

impl<R> UpdateInput<&mut R> for Pheromones
where
    R: rand::Rng,
{
    fn update_input(self, rng: &mut R) -> Self {
        if rng.random_ratio(1, DECAY_DENOMINATOR) {
            Pheromones::new(self.food - 2, self.home - 1)
        } else {
            self
        }
    }
}
