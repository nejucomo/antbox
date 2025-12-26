use antbox_geom::{Bounds, Grid};
use derive_more::Deref;
use wyrand::WyRand;

/// A facility to introduce stable pseudo-random variation per [Grid] cell
#[derive(Debug, Deref)]
pub struct WyrGrid(Grid<WyRand>);

impl WyrGrid {
    /// Initialize a new wyrgrid with random cell states
    pub fn new<R: rand::Rng>(bounds: Bounds, mut rng: R) -> Self {
        let mut v = Vec::with_capacity(bounds.area());
        for _ in 0..bounds.area() {
            v.push(WyRand::new(rng.random()));
        }
        WyrGrid(Grid::new(bounds, v))
    }
}
