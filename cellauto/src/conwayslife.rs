use crate::{Generation, Ruleset};

/// The [Ruleset] for [Conway's Life](https://conwaylife.com/wiki/Conway's_Game_of_Life)
#[derive(Copy, Clone, Debug, Default)]
pub struct ConwaysLife;

impl Ruleset for ConwaysLife {
    fn next_generation(&self, g: &Generation) -> Generation {
        let mut nextgen: Generation = Generation::from(g.bounds());
        for pt in nextgen.bounds().iter_points() {
            for npt in pt.neighbors() {
                if g[npt].is_alive() {
                    nextgen[pt].count_neighbor();
                }
            }
        }
        nextgen
    }
}
