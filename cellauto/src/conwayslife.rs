use crate::{Generation, Ruleset};

/// The [Ruleset] for [Conway's Life](https://conwaylife.com/wiki/Conway's_Game_of_Life)
#[derive(Copy, Clone, Debug, Default)]
pub struct ConwaysLife;

impl Ruleset for ConwaysLife {
    fn next_generation(&self, g: &Generation) -> Generation {
        let mut nextgen: Generation = Generation::from(g.bounds());
        for pt in nextgen.bounds().iter_points() {
            let nextcell = &mut nextgen[pt];
            for npt in pt.neighbors() {
                if g[npt].is_alive() {
                    nextcell.count_neighbor();
                }
            }
            nextcell.set_alive(match nextcell.neighbor_count() {
                // underpopulation or over-exposure:
                0 | 1 | 4..=8 => false,

                // With 2 neighbors, we can stay alive only if we were previously alive:
                2 => g[pt].is_alive(),

                // birth or persistence
                3 => true,

                // Only up to 8 cells are possible:
                n => panic!("incoherent neighbor count: {n:?}"),
            });
        }
        nextgen
    }
}
