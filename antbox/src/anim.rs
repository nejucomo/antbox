use std::cmp::Ordering::{Equal, Greater, Less};
use std::f32::consts::PI;

use antbox_geom::{BoundPoint, Bounds, Grid};
use antbox_state::{GenParams, State};
use antbox_trig::{Angle, TrigVec};
use mealy_machine::{IntoNext, UpdateInput};
use rand::Rng as _;
use rand::rngs::StdRng;
use speedy2d::Graphics2D;
use speedy2d::dimen::Vec2;

use crate::{Tick, colors};

const TICKS_PER_CONWAY: usize = 10;

/// Encapsulate all graphics rendering for a [State]
#[derive(Debug)]
pub(crate) struct AnimationState {
    rng: StdRng,
    antbox: State,
    /// Ticks until we advance antbox [State]
    ticksleft: usize,
    /// Intermediate seed pods transitioning towards `antbox` state
    foodtransients: Grid<(u8, bool)>,
}

impl AnimationState {
    pub fn new(gp: GenParams) -> Self {
        let (rng, antbox) = gp.generate_state();
        let foodtransients = Grid::from(antbox.bounds);
        AnimationState {
            rng,
            antbox,
            ticksleft: 0, // We always advance one Conway on first update
            foodtransients,
        }
    }

    fn reconcile_transients(&mut self) {
        for (pt, (nc, alive)) in self.foodtransients.iter_mut() {
            if self.rng.random_ratio(1, 2) {
                let targetnc = self.antbox.food.neighbor_counts()[pt];
                let (next_nc, next_alive) = match targetnc.cmp(nc) {
                    Less => (*nc - 1, false),
                    Greater => (*nc + 1, false),
                    Equal => (*nc, true),
                };
                *nc = next_nc;
                *alive = next_alive;
            }
        }
    }
}

impl UpdateInput<Tick> for AnimationState {
    fn update_input(mut self, _: Tick) -> Self {
        if self.ticksleft == 0 {
            self.antbox = self.antbox.into_next();
            self.ticksleft = TICKS_PER_CONWAY;
        } else {
            self.ticksleft -= 1;
        }
        self.reconcile_transients();
        self
    }
}

#[derive(Debug)]
struct RenderMetrics {
    view_size: Vec2,
    food_cell_bounds: Vec2,
    food_cell_radius: f32,
}

impl AnimationState {
    pub(crate) fn draw(&self, gfx: &mut Graphics2D, view_size: Vec2) {
        let food_cell_bounds = {
            let bounds = self.antbox.food.bounds();
            let w32 = bounds.width as f32;
            let h32 = bounds.height as f32;

            Vec2::new(view_size.x / w32, view_size.y / h32)
        };

        let rm = &RenderMetrics {
            view_size,
            food_cell_bounds,
            food_cell_radius: food_cell_bounds.x.min(food_cell_bounds.y) / 2.0,
        };

        self.draw_background(gfx, rm);
        self.draw_food_pods(gfx, rm);
        self.draw_wire_frame(gfx, rm);
    }

    fn draw_background(&self, g: &mut Graphics2D, _: &RenderMetrics) {
        g.clear_screen(colors::BACKGROUND);
    }

    fn draw_wire_frame(&self, g: &mut Graphics2D, rm: &RenderMetrics) {
        let Bounds { width, height } = self.antbox.food.bounds();
        for col in 0..width {
            let x = (col as f32) * rm.food_cell_bounds.x;
            g.draw_line((x, 0.0), (x, rm.view_size.y), 1.0, colors::WIRE_FRAME);
        }
        for row in 0..height {
            let y = (row as f32) * rm.food_cell_bounds.y;
            g.draw_line((0.0, y), (rm.view_size.x, y), 1.0, colors::WIRE_FRAME);
        }
    }

    fn draw_food_pods(&self, g: &mut Graphics2D, rm: &RenderMetrics) {
        let crad = rm.food_cell_radius * 0.9; // crowded flavor

        for (pt, center) in self.iter_pts_and_centers(rm) {
            let (cnt, alive) = self.foodtransients[pt];
            if cnt > 0 {
                let cellrotation = Angle::from(center.magnitude());
                let berrycolor = colors::food_neighbor_count(cnt);

                let c = cnt as f32;
                let theta = PI / c;

                let berryrad = {
                    let magic_sauce = (theta * 0.71).sin();
                    crad * (magic_sauce / (1.1 + magic_sauce))
                };

                let spoke = TrigVec::new(PI / c, 0.8 * crad - berryrad);

                g.draw_circle(center, crad, colors::SEEDPOD);
                for berry in 0..cnt {
                    let bspoke = spoke.rotate(cellrotation + 2.0 * theta * berry as f32);
                    g.draw_circle(center + bspoke.into_vec2(), berryrad, berrycolor);
                }
                if alive {
                    g.draw_circle(center, rm.food_cell_radius / 5.0, colors::FOODLIFE);
                }
            }
        }
    }

    fn iter_pts_and_centers(&self, rm: &RenderMetrics) -> impl Iterator<Item = (BoundPoint, Vec2)> {
        let Vec2 { x: cellw, y: cellh } = rm.food_cell_bounds;

        self.antbox.food.bounds().iter_points().map(move |pt| {
            (
                pt,
                Vec2::new(
                    cellw / 2.0 * (1 + 2 * pt.x()) as f32,
                    cellh / 2.0 * (1 + 2 * pt.y()) as f32,
                ),
            )
        })
    }
}
