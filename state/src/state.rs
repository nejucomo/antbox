use antbox_clife::{ConwayGrid, ConwayMachine};
use antbox_geom::{BoundPoint, DirSet, Direction, Grid};
use derive_more::{Deref, From, Into};
use movestate::toolkit::Cycler;
use movestate::{IntoNext as _, UpdateInput};

use crate::randutil::ShuffleIntoVec as _;
use crate::{Ant, Pheromone, Spot, SteppedUpon as _};

const TICKS_PER_CONWAY: usize = 50;

/// The `antbox` functional, I/O-free [State]
#[derive(Debug, From, Into, Deref)]
pub struct State {
    /// The generation count
    generation: usize,
    /// The grid of objects
    #[deref]
    grid: Grid<Spot>,
    /// The state influencing food growth:
    clife: Cycler<ConwayMachine<bool>>,
}

impl State {
    /// Construct a new state from a [Grid]
    pub fn new(grid: Grid<Spot>, clife: Grid<bool>) -> Self {
        State {
            generation: 0,
            grid,
            clife: Cycler::new(ConwayMachine::new(clife), TICKS_PER_CONWAY),
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

    pub fn food_is_growing(&self, pt: BoundPoint) -> bool {
        self.clife[pt]
    }

    pub fn life_and_neighbors(&self, pt: BoundPoint) -> (bool, usize) {
        self.clife.life_and_neighbors(pt)
    }

    pub(crate) fn move_ant(&mut self, ant: Ant, src: BoundPoint, dst: BoundPoint) {
        if let Some(dstspot) = self.grid[dst].stepped_upon_by(ant) {
            self.grid[dst] = dstspot;

            let shadow = self.grid[src].take_object();
            assert_eq!(Some(crate::Object::Ant(ant)), shadow);
        }
    }
}

// Janky hack to update in two passes; TODO: API impedance mismatch.
struct LifePhase;
struct GridPhase;

impl<R> UpdateInput<&mut R> for State
where
    R: rand::Rng,
{
    fn update_input(self, rng: &mut R) -> Self {
        self.update_input(LifePhase).update_input((GridPhase, rng))
    }
}

impl UpdateInput<LifePhase> for State {
    fn update_input(self, _: LifePhase) -> Self {
        State {
            generation: self.generation + 1,
            grid: self.grid,
            clife: self.clife.into_next(),
        }
    }
}

impl<R> UpdateInput<(GridPhase, &mut R)> for State
where
    R: rand::Rng,
{
    fn update_input(mut self, (_, rng): (GridPhase, &mut R)) -> Self {
        // TODO: Performance: this is a full copy of all points! Can we do a cheaper permutation 0..area?
        let ptspots = self
            .iter()
            .map(|(pt, &spot)| (pt, spot))
            .shuffle_into_vec(rng);

        for (pt, spot) in ptspots {
            self[pt] = spot.update_input((rng, &self, pt));
        }

        self
    }
}
