use antbox_geom::Bounds;
use derive_more::{From, Into};
use derive_new::new;
use rand::Rng;
use rand::distr::Distribution;

use crate::Generation;

/// A [Distribution] for generating a [Generation]
#[derive(Copy, Clone, Debug, From, Into, new)]
pub struct GenGen {
    cell_prob: f64,
    bounds: Bounds,
}

impl Distribution<Generation> for GenGen {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Generation {
        let mut g = Generation::from(self.bounds);
        for (_, cellptr) in g.iter_mut() {
            cellptr.set_alive(rng.random_bool(self.cell_prob));
        }
        for pt in g.bounds().iter_points() {
            for npt in pt.neighbors() {
                if g[npt].is_alive() {
                    g[pt].count_neighbor();
                }
            }
        }
        g
    }
}
