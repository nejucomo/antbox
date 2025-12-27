use antbox_geom::{Bounds, Grid};
use clap::Args;
use derive_more::{From, Into};
use derive_new::new;
use rand::Rng;
use rand::distr::Distribution;

use crate::{AntHole, Field, Spot, State};

/// A [Distribution] for generating a [State]
#[derive(Args, Copy, Clone, Debug, From, Into, new)]
pub struct GenParams {
    /// The probability a given cell will have growth (Conway's Life)
    #[clap(long, default_value = "0.7", help_heading = "Generation Parameters")]
    pub cell_prob: f64,
    /// The size of the grid
    #[clap(long, default_value = "70x40", help_heading = "Generation Parameters")]
    pub grid_size: Bounds,
}

impl GenParams {
    /// Generate the initial state from the parameters
    pub fn generate_state<R: Rng>(self, rng: &mut R) -> State {
        self.sample(rng)
    }
}

impl Distribution<State> for GenParams {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> State {
        State::new(Field::new(self.sample(rng), self.sample(rng)))
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
