use antbox_geom::{Bounds, Grid};
use clap::Args;
use derive_more::{From, Into};
use derive_new::new;
use rand::Rng;
use rand::distr::Distribution;

use crate::{AntHole, Spot, State};

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
        State::new(self.sample(rng), self.sample(rng))
    }
}

impl Distribution<Grid<Spot>> for GenParams {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Grid<Spot> {
        let mut g = Grid::from(self.grid_size);

        // Add one ant-hole
        let pt = g.bounds().sample(rng);
        g[pt] = Spot::from(AntHole::default());

        g
    }
}

impl Distribution<Grid<bool>> for GenParams {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Grid<bool> {
        let area = self.grid_size.area();
        let mut cells = Vec::with_capacity(area);
        for _ in 0..area {
            cells.push(rng.random_bool(self.cell_prob));
        }
        Grid::new(self.grid_size, cells)
    }
}
