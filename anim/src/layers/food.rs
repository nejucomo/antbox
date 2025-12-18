use std::cmp::Ordering::{Equal, Greater, Less};
use std::f32::consts::{PI, TAU};

use antbox_clife::ConwayGrid as _;
use antbox_geom::{BoundPoint, Bounds, Grid};
use antbox_state::State as AntboxState;
use antbox_trig::{Angle, TrigVec};
use derive_new::new;
use mealy_machine::UpdateInput;
use rand::distr::Distribution;
use rand_distr::LogNormal;
use speedy2d::dimen::Vec2;

use crate::{Drawable, GfxLayout, RectExt as _, colors};

/// The layer of animated food state
#[derive(Debug)]
pub struct FoodDecoration {
    grid: Grid<FoodCell>,
    hsdist: LogNormal<f32>,
    hotspots: Vec<BoundPoint>,
}

#[derive(Copy, Clone, Debug, Default, new)]
struct FoodCell {
    seeds: Option<u8>,
    alive: bool,
}

impl From<Bounds> for FoodDecoration {
    fn from(b: Bounds) -> Self {
        FoodDecoration {
            grid: Grid::from(b),
            hsdist: LogNormal::new(1.5, 0.8).unwrap(),
            hotspots: vec![],
        }
    }
}

impl<R> UpdateInput<(&mut R, &AntboxState)> for FoodDecoration
where
    R: rand::Rng,
{
    fn update_input(mut self, (rng, ast): (&mut R, &AntboxState)) -> Self {
        // Add new hotspots
        if self.hotspots.is_empty() || rng.random_ratio(1, 23) {
            self.hotspots.push(self.grid.bounds().sample(rng));
        }

        // update hotspots:
        for hotspot in self.hotspots.iter().copied() {
            if rng.random_ratio(1, 3) {
                break;
            }

            let pt = {
                // Stick it in a fn:
                let hsf = Vec2::new(hotspot.x() as f32, hotspot.y() as f32);
                let delta = TrigVec::new(rng.random_range(0f32..TAU), self.hsdist.sample(rng));
                let Vec2 { x, y } = hsf + delta;
                let bounds = self.grid.bounds();
                let x = x.rem_euclid(bounds.width as f32) as usize;
                let y = y.rem_euclid(bounds.width as f32) as usize;
                self.grid.bounds().bind((x, y)).unwrap()
            };

            let cell = &mut self.grid[pt];
            let (target_life, target_nc) = ast.life_and_neighbors(pt);

            let (optseeds, alive) = if let Some(seeds) = cell.seeds {
                match (target_nc as u8).cmp(&seeds) {
                    Less => (Some(seeds - 1), false),
                    Greater => (Some(seeds + 1), false),
                    // Once they are equal, defer to the antbox life status:
                    Equal => (Some(seeds), target_life),
                }
            } else if target_life || target_nc > 0 {
                (Some(0), false)
            } else {
                (None, false)
            };

            *cell = FoodCell::new(optseeds, alive);
        }

        // drop a hotspot
        if self.hotspots.len() > 7 || rng.random_ratio(1, 29) {
            self.hotspots.remove(0);
        }

        self
    }
}

impl Drawable for &FoodDecoration {
    fn draw_on(self, g: &mut GfxLayout<'_>) {
        let gl = g.grid_layout;

        let crad = gl.cell_radius * 0.9; // crowded flavor

        for (pt, rect) in gl.iter_pts_and_rects() {
            let FoodCell { seeds, alive } = self.grid[pt];
            if let Some(seeds) = seeds {
                let center = rect.center();
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
                    g.draw_circle(center, crad, colors::FOOD_LIFE);
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
