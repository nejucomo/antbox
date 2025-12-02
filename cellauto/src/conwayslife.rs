use antbox_geom::Grid;

use crate::{Generation, Ruleset};

/// The [Ruleset] for [Conway's Life](https://conwaylife.com/wiki/Conway's_Game_of_Life)
#[derive(Copy, Clone, Debug, Default)]
pub struct ConwaysLife;

pub fn conways_rule(already_alive: bool, neighbor_count: u8) -> bool {
    match neighbor_count {
        // underpopulation or over-exposure:
        0 | 1 | 4..=8 => false,

        // With 2 neighbors, we can stay alive only if we were previously alive:
        2 => already_alive,

        // birth or persistence
        3 => true,

        // Only up to 8 cells are possible:
        n => panic!("incoherent neighbor count: {n:?}"),
    }
}

pub fn neighbor_counts(g: &Generation) -> Grid<u8> {
    let mut neighbors = Grid::from(g.bounds());
    for (pt, c) in g.iter() {
        if c.is_alive() {
            for npt in pt.neighbors() {
                neighbors[npt] += 1;
            }
        }
    }
    neighbors
}

pub fn next_gen_from_neighbor_counts(g: &Generation, nc: &Grid<u8>) -> Generation {
    let mut nextgen = Generation::from(g.bounds());

    for (pt, c) in nextgen.iter_mut() {
        c.set_alive(conways_rule(g[pt].is_alive(), nc[pt]));
    }

    nextgen
}

impl Ruleset for ConwaysLife {
    fn next_generation(&self, g: &Generation) -> Generation {
        let nc = neighbor_counts(g);
        next_gen_from_neighbor_counts(g, &nc)
    }
}

#[test]
fn twiddler() {
    use antbox_geom::Bounds;

    let mut gs = vec![Generation::from(Bounds::new(5, 5))];
    gs[0][(2, 1)].set_alive(true);
    gs[0][(2, 2)].set_alive(true);
    gs[0][(2, 3)].set_alive(true);

    dbg!(&gs[0], neighbor_counts(&gs[0]));

    let nextgen = ConwaysLife.next_generation(&gs[0]);
    gs.push(nextgen);

    let nextgen = ConwaysLife.next_generation(&gs[1]);
    gs.push(nextgen);

    assert_ne!(gs[0], gs[1]);
    assert_eq!(gs[0], gs[2]);
}
