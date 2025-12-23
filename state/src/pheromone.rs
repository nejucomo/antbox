use std::ops::{Add, Sub};

use derive_new::new;
use movestate::take_into::TakeIntoNext;

use self::Pheromone::*;

const DECAY_DENOMINATOR: u32 = 37;

/// The local levels of the [Pheromone]s
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, new)]
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

    /// Clamp each magnitude to 1
    pub fn clamp(self) -> Self {
        Pheromones {
            food: self.food.clamp(0, 1),
            home: self.home.clamp(0, 1),
        }
    }
}

impl From<Pheromone> for Pheromones {
    fn from(ph: Pheromone) -> Self {
        match ph {
            Food => Pheromones { food: 1, home: 0 },
            Home => Pheromones { food: 0, home: 1 },
        }
    }
}

impl<R> TakeIntoNext<&mut R> for Pheromones
where
    R: rand::Rng,
{
    type Next = Self;

    fn take_into_next(self, rng: &mut R) -> Self {
        if !self.is_empty() && rng.random_ratio(1, DECAY_DENOMINATOR) {
            Pheromones::new(self.food - 2.min(self.food), self.home - 1.min(self.home))
        } else {
            self
        }
    }
}

impl Add for Pheromones {
    type Output = Pheromones;

    fn add(self, other: Self) -> Self::Output {
        Pheromones {
            food: self.food.saturating_add(other.food),
            home: self.home.saturating_add(other.home),
        }
    }
}

impl Sub for Pheromones {
    type Output = Pheromones;

    fn sub(self, other: Self) -> Self::Output {
        Pheromones {
            food: self.food.saturating_sub(other.food),
            home: self.home.saturating_sub(other.home),
        }
    }
}
