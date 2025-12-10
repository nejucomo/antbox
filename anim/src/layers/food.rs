use std::cmp::Ordering::{Equal, Greater, Less};
use std::f32::consts::PI;

use antbox_geom::{Bounds, Grid};
use antbox_state::State as AntboxState;
use antbox_trig::{Angle, TrigVec};
use derive_new::new;
use mealy_machine::UpdateInput;
use rand::Rng as _;
use rand::rngs::StdRng;

use crate::{Drawable, GfxLayout, TICKS_PER_CONWAY, colors};

/// The layer of animated food state
#[derive(Debug)]
pub struct Food(Grid<FoodCell>);

#[derive(Copy, Clone, Debug, Default, new)]
struct FoodCell {
    seeds: u8,
    alive: bool,
}

impl From<Bounds> for Food {
    fn from(b: Bounds) -> Self {
        Food(Grid::from(b))
    }
}

impl UpdateInput<(&mut StdRng, &AntboxState)> for Food {
    fn update_input(mut self, (rng, ast): (&mut StdRng, &AntboxState)) -> Self {
        for (pt, cell) in self.0.iter_mut() {
            if rng.random_ratio(2, TICKS_PER_CONWAY.try_into().unwrap()) {
                let target_nc = ast.food.neighbor_counts()[pt];
                let newcell = match target_nc.cmp(&cell.seeds) {
                    Less => FoodCell::new(cell.seeds - 1, false),
                    Greater => FoodCell::new(cell.seeds + 1, false),
                    // Once they are equal, defer to the antbox life status:
                    Equal => FoodCell::new(cell.seeds, ast.food.life()[pt].is_alive()),
                };
                *cell = newcell;
            }
        }
        self
    }
}

impl Drawable for &Food {
    fn draw_on(self, g: &mut GfxLayout<'_>) {
        let gl = g.grid_layout;

        let crad = gl.cell_radius * 0.9; // crowded flavor

        for (pt, center) in gl.iter_pts_and_centers() {
            let FoodCell { seeds, alive } = self.0[pt];
            if alive || seeds > 0 {
                let cellrotation = Angle::from(center.magnitude());
                let berrycolor = colors::food_neighbor_count(seeds);

                let c = seeds as f32;
                let theta = PI / c;

                let berryrad = {
                    let magic_sauce = (theta * 0.71).sin();
                    let seedf = if seeds == 1 {
                        center.magnitude_squared().rem_euclid(1.0).powf(0.3)
                    } else {
                        1.0
                    };
                    crad * seedf * (magic_sauce / (1.1 + magic_sauce))
                };

                let spoke = TrigVec::new(PI / c, 0.8 * crad - berryrad);

                if alive {
                    g.draw_circle(center, crad, colors::FOODLIFE);
                }
                g.draw_circle(center, crad * 0.9, colors::SEEDPOD);
                for berry in 0..seeds {
                    let mut bspoke = spoke.rotate(cellrotation + 2.0 * theta * berry as f32);

                    if seeds == 1 {
                        bspoke = bspoke.scale(center.magnitude().rem_euclid(1.0));
                    }

                    g.draw_circle(center + bspoke.into_vec2(), berryrad, berrycolor);
                }
            }
        }
    }
}
