use antbox_clife::{Cell, ConwaysLife, Generation};
use antbox_geom::Bounds;
use clap::Args;
use derive_more::{From, Into};
use derive_new::new;
use rand::Rng;
use rand::distr::Distribution;

use crate::State;

/// A [Distribution] for generating a [State]
#[derive(Args, Copy, Clone, Debug, From, Into, new)]
pub struct GenParams {
    #[clap(long, default_value = "0.7", help_heading = "Generation Parameters")]
    cell_prob: f64,
    #[clap(long, default_value = "70x40", help_heading = "Generation Parameters")]
    grid_size: Bounds,
}

impl GenParams {
    /// Generate the initial state from the parameters
    pub fn generate_state<R: Rng>(self, rng: &mut R) -> State {
        self.sample(rng)
    }
}

impl Distribution<State> for GenParams {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> State {
        State::new(self.sample(rng))
    }
}

impl Distribution<ConwaysLife> for GenParams {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ConwaysLife {
        let g: Generation = self.sample(rng);
        ConwaysLife::new(g)
    }
}

impl Distribution<Generation> for GenParams {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Generation {
        let area = self.grid_size.area();
        let mut cells = Vec::with_capacity(area);
        for _ in 0..area {
            cells.push(self.sample(rng));
        }

        Generation::new(self.grid_size, cells)
    }
}

impl Distribution<Cell> for GenParams {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        Cell::from(rng.random_bool(self.cell_prob))
    }
}
