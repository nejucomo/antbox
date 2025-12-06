use antbox_cellauto::{Cell, ConwaysLife, Generation};
use antbox_geom::Bounds;
use clap::Args;
use derive_more::{From, Into};
use derive_new::new;
use rand::distr::Distribution;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng as _};

use crate::State;

/// A [Distribution] for generating a [State]
#[derive(Args, Copy, Clone, Debug, From, Into, new)]
pub struct GenParams {
    #[clap(long, default_value = "0", help_heading = "Generation Parameters")]
    seed: u64,
    #[clap(long, default_value = "0.7", help_heading = "Generation Parameters")]
    cell_prob: f64,
    #[clap(
        long,
        default_value = "180x120",
        help_heading = "Generation Parameters"
    )]
    grid_size: Bounds,
}

impl GenParams {
    /// Generate the initial state from the parameters
    pub fn generate_state(self) -> State {
        let mut rng = StdRng::seed_from_u64(self.seed);
        self.sample(&mut rng)
    }
}

impl Distribution<State> for GenParams {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> State {
        State::new(0, self.sample(rng))
    }
}

impl Distribution<ConwaysLife> for GenParams {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ConwaysLife {
        let g: Generation = self.sample(rng);
        ConwaysLife::from(g)
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
