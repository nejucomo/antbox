use antbox_clife::ConwayMachine;
use antbox_geom::{BoundPoint, DirSet, Direction, Grid};
use derive_more::{Deref, From, Into};
use mealy_machine::toolkit::Cycler;
use mealy_machine::{IntoNext as _, UpdateInput};

use crate::randutil::ShuffleIntoVec as _;
use crate::{Ant, Objectish as _, Pheromone, Spot};

const ANTS_PER_CONWAY_TICK: usize = 50;

/// The `antbox` functional, I/O-free [State]
#[derive(Debug, From, Into, Deref)]
pub struct State {
    /// The generation count
    generation: usize,
    /// The grid of objects
    #[deref]
    grid: Cycler<ConwayMachine<Spot>>,
}

impl State {
    /// Construct a new state from a [Grid]
    pub fn new(grid: Grid<Spot>) -> Self {
        State {
            generation: 0,
            grid: Cycler::new(ConwayMachine::new(grid), ANTS_PER_CONWAY_TICK),
        }
    }

    /// Return the directions from `pt` which have the `greatest`/weakest magnitude of `ph`
    pub fn pheromone_gradient(&self, pt: BoundPoint, ph: Pheromone, greatest: bool) -> DirSet {
        let it = Direction::each().map(|d| self[pt + d].pheromone_magnitude(ph));

        let best = if greatest {
            it.max()
        } else {
            it.max_by_key(|m| u8::MAX - m)
        }
        .unwrap();

        self.directions_where(pt, |spot| spot.pheromone_magnitude(ph) == best)
    }

    fn step_ants<R>(mut self, rng: &mut R) -> Self
    where
        R: rand::Rng,
    {
        let ants = self
            .iter()
            .filter_map(|(pt, spot)| spot.as_ant().map(|ant| (pt, ant)))
            .shuffle_into_vec(rng);

        for (pt, ant) in ants {
            ant.sense_then_step(&mut self, rng, pt);
        }

        self
    }

    pub(crate) fn move_ant(&mut self, ant: Ant, src: BoundPoint, dst: BoundPoint) {
        if self.grid[dst].stepped_upon(ant) {
            let shadow = self.grid[src].take_object();
            assert_eq!(Some(crate::Object::Ant(ant)), shadow);
        }
    }
}

impl<R> UpdateInput<&mut R> for State
where
    R: rand::Rng,
{
    fn update_input(self, rng: &mut R) -> Self {
        let generation = self.generation + 1;

        State {
            generation,
            grid: self.grid.into_next(),
        }
        .step_ants(rng)
    }
}
