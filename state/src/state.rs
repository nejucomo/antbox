use antbox_clife::ConwayGrid as _;
use antbox_geom::{BoundPoint, DirSet, Direction, Grid};
use derive_more::{Deref, From, Into};
use derive_new::new;
use mealy_machine::UpdateInput;

use crate::randutil::ShuffleIntoVec as _;
use crate::{Ant, Objectish as _, Pheromone, Spot};

/// The `antbox` functional, I/O-free [State]
#[derive(Debug, From, Into, Deref, new)]
pub struct State {
    /// The generation count
    #[new(default)]
    generation: usize,
    /// The grid of objects
    #[deref]
    grid: Grid<Spot>,
}

impl State {
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
        State {
            generation: self.generation + 1,
            grid: self.grid.conway_step(),
        }
        .step_ants(rng)
    }
}
