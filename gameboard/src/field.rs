use std::collections::BTreeSet;

use antbox_clife::{ConwayGrid, ConwayMachine};
use antbox_grid::{BoundPoint, DirSet, Direction, Grid};
use derive_more::{Deref, From, Into};
use movestate::TakeIntoNext;

use crate::gencount::{Cycler, GenerationCount};
use crate::randutil::ShuffleIntoVec as _;
use crate::spotupdate::SpotUpdate;
use crate::{Ant, Pheromone, Spot, SteppedUpon as _};

const TICKS_PER_CONWAY: usize = 50;

/// The field of play for the ants
#[derive(Debug, From, Into, Deref)]
pub struct Field {
    /// The grid of objects
    #[deref]
    grid: Grid<Spot>,
    /// The state influencing food growth:
    clife: Cycler<ConwayMachine<bool>>,
}

impl Field {
    /// Construct a new [Field]
    pub fn new(grid: Grid<Spot>, clife: Grid<bool>) -> Self {
        Field {
            grid,
            clife: Cycler::new(ConwayMachine::new(clife), TICKS_PER_CONWAY),
        }
    }

    /// Return the directions from `pt` which have the `greatest`/weakest magnitude of `ph`
    pub fn pheromone_gradient(&self, pt: BoundPoint, ph: Pheromone, greatest: bool) -> DirSet {
        let it = Direction::each().map(|d| self[pt + d].pheromones().magnitude(ph));

        let best = if greatest {
            it.max()
        } else {
            it.max_by_key(|m| u8::MAX - m)
        }
        .unwrap();

        self.directions_where(pt, |spot| spot.pheromones().magnitude(ph) == best)
    }

    /// Whether or not food is growing here
    pub fn food_is_growing(&self, pt: BoundPoint) -> bool {
        self.clife[pt]
    }

    /// The growth status and growth-neighbors here
    pub fn growth_and_neighbors(&self, pt: BoundPoint) -> (bool, usize) {
        self.clife.life_and_neighbors(pt)
    }

    pub(crate) fn move_ant(&mut self, ant: Ant, dst: BoundPoint) -> bool {
        if let Some(dstspot) = self.grid[dst].stepped_upon_by(ant) {
            self.grid[dst] = dstspot;
            true
        } else {
            false
        }
    }
}

// Janky hack to update in two passes; TODO: API impedance mismatch.
struct LifePhase;
struct GridPhase;

impl<R> TakeIntoNext<(GenerationCount, &mut R)> for Field
where
    R: rand::Rng,
{
    type Next = Self;

    fn take_into_next(self, (gc, rng): (GenerationCount, &mut R)) -> Self::Next {
        self.take_into_next((LifePhase, gc))
            .take_into_next((GridPhase, rng))
    }
}

impl TakeIntoNext<(LifePhase, GenerationCount)> for Field {
    type Next = Self;

    fn take_into_next(self, (_, gc): (LifePhase, GenerationCount)) -> Self {
        Field {
            grid: self.grid,
            clife: self.clife.take_into_next(gc),
        }
    }
}

impl<R> TakeIntoNext<(GridPhase, &mut R)> for Field
where
    R: rand::Rng,
{
    type Next = Self;

    fn take_into_next(mut self, (_, rng): (GridPhase, &mut R)) -> Self {
        // TODO: Performance: this is a full copy of all points! Can we do a cheaper permutation 0..area?
        let pts = self.bounds().iter_points().shuffle_into_vec(rng);
        let mut steppedonto = BTreeSet::default();

        for pt in pts {
            if !steppedonto.remove(&pt) {
                let (newspot, optstep) =
                    self.grid[pt].take_into_next(SpotUpdate::new(rng, &mut self, pt));
                self.grid[pt] = newspot;
                if let Some(dst) = optstep {
                    assert!(steppedonto.insert(dst));
                }
            }
        }

        self
    }
}
