use antbox_cellauto::{Cell, ConwaysLife, Generation};
use antbox_geom::Bounds;
use derive_more::{From, Into};
use derive_new::new;
use rand::Rng;
use rand::distr::Distribution;

use crate::State;

/// A [Distribution] for generating a [State]
#[derive(Copy, Clone, Debug, From, Into, new)]
pub struct GenParams {
    cell_prob: f64,
    bounds: Bounds,
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
        let area = self.bounds.area();
        let mut cells = Vec::with_capacity(area);
        for _ in 0..area {
            cells.push(self.sample(rng));
        }

        Generation::new(self.bounds, cells)
    }
}

impl Distribution<Cell> for GenParams {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        Cell::from(rng.random_bool(self.cell_prob))
    }
}
