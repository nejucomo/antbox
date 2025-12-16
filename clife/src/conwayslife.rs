use antbox_geom::{Bounds, Grid};
use derive_more::{From, Into};
use mealy_machine::IntoNext;

use crate::Generation;

/// A wrapper for a [Generation] providing an [IntoNext] implementation of [Conway's Life](https://conwaylife.com/wiki/Conway's_Game_of_Life)
#[derive(Clone, Debug, From, Into, PartialEq)]
pub struct ConwaysLife {
    /// Life status
    life: Generation,
    /// Neighbor-counts for `self.life`
    nc: Grid<u8>,
}

impl ConwaysLife {
    /// Construct a new state given a `life` grid of booleans
    pub fn new(life: Generation) -> Self {
        let nc = neighbor_counts(&life);
        ConwaysLife { life, nc }
    }

    /// The [Bounds] of the [Self::life] grid
    pub fn bounds(&self) -> Bounds {
        self.life.bounds()
    }

    /// The grid of "is_alive" boolean states
    pub fn life(&self) -> &Generation {
        &self.life
    }

    /// The grid of neighbor counts for [Self::life]
    pub fn neighbor_counts(&self) -> &Grid<u8> {
        &self.nc
    }
}

impl IntoNext for ConwaysLife {
    fn into_next(self) -> Self {
        ConwaysLife::new(next_gen_from_neighbor_counts(self.life, self.nc))
    }
}

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

pub fn next_gen_from_neighbor_counts(g: Generation, nc: Grid<u8>) -> Generation {
    let mut nextgen = Generation::from(g.bounds());

    for (pt, c) in nextgen.iter_mut() {
        c.set_alive(conways_rule(g[pt].is_alive(), nc[pt]));
    }

    nextgen
}

#[test]
fn twiddler() {
    use antbox_geom::Bounds;

    let mut g0 = Generation::from(Bounds::new(5, 5));
    g0[(2, 1)].set_alive(true);
    g0[(2, 2)].set_alive(true);
    g0[(2, 3)].set_alive(true);
    dbg!(&g0, neighbor_counts(&g0));

    let mut gs = vec![ConwaysLife::new(g0)];

    // evolve two new generations:
    gs.push(gs.last().cloned().unwrap().into_next());
    gs.push(gs.last().cloned().unwrap().into_next());

    assert_ne!(gs[0], gs[1]);
    assert_eq!(gs[0], gs[2]);
}
