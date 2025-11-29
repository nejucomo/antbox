use antbox_geom::Bounds;
use derive_more::{From, Into};
use derive_new::new;
use rand::Rng;
use rand::distr::Distribution;
use try_from_unwrap::TryFromUnwrap;

use crate::{Cell, Generation};

/// A [Distribution] for generating a [Generation]
#[derive(Copy, Clone, Debug, From, Into, new)]
pub struct GenGen {
    cell_prob: f64,
    bounds: Bounds,
}

impl Distribution<Generation> for GenGen {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Generation {
        let area = usize::tfu(self.bounds.area());
        let mut cells = Vec::with_capacity(area);
        for _ in 0..area {
            cells.push(self.sample(rng));
        }

        Generation::new(self.bounds, cells)
    }
}

impl Distribution<Cell> for GenGen {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        Cell::from(rng.random_bool(self.cell_prob))
    }
}
