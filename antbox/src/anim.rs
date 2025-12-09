use std::f32::consts::PI;

use antbox_geom::{BoundPoint, Bounds};
use antbox_state::State;
use antbox_trig::{Angle, TrigVec};
use derive_new::new;
use mealy_machine::{IntoNext, UpdateInput};
use speedy2d::Graphics2D;
use speedy2d::dimen::Vec2;

use crate::{Tick, colors};

/// Encapsulate all graphics rendering for a [State]
#[derive(Debug, new)]
pub(crate) struct AnimationState {
    state: State,
}

impl UpdateInput<Tick> for AnimationState {
    fn update_input(mut self, _: Tick) -> Self {
        self.state = self.state.into_next();
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
            let bounds = self.state.food.bounds();
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
        self.draw_food_life(gfx, rm);
        self.draw_wire_frame(gfx, rm);
    }

    fn draw_background(&self, g: &mut Graphics2D, _: &RenderMetrics) {
        g.clear_screen(colors::BACKGROUND);
    }

    fn draw_wire_frame(&self, g: &mut Graphics2D, rm: &RenderMetrics) {
        let Bounds { width, height } = self.state.food.bounds();
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
            let cnt = self.state.food.neighbor_counts()[pt];
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
            }
        }
    }

    fn draw_food_life(&self, g: &mut Graphics2D, rm: &RenderMetrics) {
        for (pt, center) in self.iter_pts_and_centers(rm) {
            let cell = self.state.food.life()[pt];
            if cell.is_alive() {
                g.draw_circle(center, rm.food_cell_radius / 2.0, colors::FOODLIFE);
            }
        }
    }

    fn iter_pts_and_centers(&self, rm: &RenderMetrics) -> impl Iterator<Item = (BoundPoint, Vec2)> {
        let Vec2 { x: cellw, y: cellh } = rm.food_cell_bounds;

        self.state.food.bounds().iter_points().map(move |pt| {
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
