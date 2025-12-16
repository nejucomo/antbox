use antbox_geom::Grid;

use crate::{ConwayCell, conways_rule};

/// A grid which can evolve via Conway's Life
pub trait ConwayGrid: Sized {
    /// Transform the current state into the next Conway's Life state
    fn conway_step(self) -> Self;
}

impl<C> ConwayGrid for Grid<C>
where
    C: ConwayCell,
{
    fn conway_step(self) -> Self {
        let life = life_grid(&self);
        let ncnts = neighbor_counts(&life);
        next_gen_from_neighbor_counts(self, life, ncnts)
    }
}

fn life_grid<C>(g: &Grid<C>) -> Grid<bool>
where
    C: ConwayCell,
{
    let mut lg = Grid::from(g.bounds());
    for (pt, c) in g.iter() {
        lg[pt] = c.is_alive();
    }
    lg
}

fn neighbor_counts(lg: &Grid<bool>) -> Grid<u8> {
    let mut neighbors = Grid::from(lg.bounds());
    for (pt, c) in lg.iter() {
        if c.is_alive() {
            for npt in pt.neighbors() {
                neighbors[npt] += 1;
            }
        }
    }
    neighbors
}

fn next_gen_from_neighbor_counts<C>(mut g: Grid<C>, life: Grid<bool>, nc: Grid<u8>) -> Grid<C>
where
    C: ConwayCell,
{
    for (pt, c) in g.iter_mut() {
        c.set_alive(conways_rule(life[pt], nc[pt]));
    }
    g
}

#[test]
fn twiddler() {
    use antbox_geom::Bounds;

    let mut gs: Vec<Grid<bool>> = vec![Grid::from(Bounds::new(5, 5))];
    gs[0][(2, 1)].set_alive(true);
    gs[0][(2, 2)].set_alive(true);
    gs[0][(2, 3)].set_alive(true);
    dbg!(&gs[0], neighbor_counts(&gs[0]));

    // evolve two new generations:
    gs.push(gs.last().cloned().unwrap().conway_step());
    gs.push(gs.last().cloned().unwrap().conway_step());

    assert_ne!(gs[0], gs[1]);
    assert_eq!(gs[0], gs[2]);
}
