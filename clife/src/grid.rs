use antbox_grid::{Grid, GridCoord};

use crate::{ConwayCell, conways_rule};

/// A grid which can evolve via Conway's Life
pub trait ConwayGrid: Sized {
    /// Transform the current state into the next Conway's Life state
    fn conway_step(self) -> Self;

    /// Get the life status and neighbor count at `pt` of the current state
    fn life_and_neighbors(&self, pt: GridCoord) -> (bool, usize);
}

impl<C> ConwayGrid for Grid<C>
where
    C: ConwayCell,
{
    fn conway_step(mut self) -> Self {
        let life = life_grid(&self);
        for (pt, cell) in self.iter_mut() {
            let (alive, nc) = life.life_and_neighbors(pt);
            cell.set_alive(conways_rule(alive, nc))
        }
        self
    }

    fn life_and_neighbors(&self, pt: GridCoord) -> (bool, usize) {
        (
            self[pt].is_alive(),
            pt.neighbors().filter(|&npt| self[npt].is_alive()).count(),
        )
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

#[test]
fn twiddler() {
    use antbox_grid::Bounds;

    let mut gs: Vec<Grid<bool>> = vec![Grid::from(Bounds::new(5, 5))];
    gs[0][(2, 1)].set_alive(true);
    gs[0][(2, 2)].set_alive(true);
    gs[0][(2, 3)].set_alive(true);

    // evolve two new generations:
    gs.push(gs.last().cloned().unwrap().conway_step());
    gs.push(gs.last().cloned().unwrap().conway_step());

    dbg!(&gs);
    assert_ne!(gs[0], gs[1]);
    assert_eq!(gs[0], gs[2]);
}
