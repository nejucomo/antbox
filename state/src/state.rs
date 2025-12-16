use antbox_clife::ConwayGrid as _;
use antbox_geom::{BoundPoint, Bounds, Grid};
use derive_more::{From, Into};
use derive_new::new;
use mealy_machine::IntoNext;

use crate::Spot;

/// The `antbox` functional, I/O-free [State]
#[derive(Debug, From, Into, new)]
pub struct State {
    /// The generation count
    #[new(default)]
    generation: usize,
    /// The grid of objects
    grid: Grid<Spot>,
}

impl State {
    /// Get the state's [Bounds]
    pub fn bounds(&self) -> Bounds {
        self.grid.bounds()
    }

    /// Return the food's life and neighbor count for the given pt
    pub fn food_life_and_neighbors(&self, pt: BoundPoint) -> (bool, usize) {
        self.grid.life_and_neighbors(pt)
    }
}

impl IntoNext for State {
    fn into_next(self) -> Self {
        State {
            generation: self.generation + 1,
            grid: self.grid.conway_step(),
        }
    }
}
