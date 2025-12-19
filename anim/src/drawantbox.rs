use std::f32::consts::{FRAC_1_SQRT_2, PI, TAU};

use antbox_state::{Ant, AntHole, Object, SeedPod, Spot, State as AntboxState};
use antbox_trig::{Angle, TrigVec};
use speedy2d::shape::Rect;

use crate::colors::{self, ANT, ANT_HOLE_ENTRANCE, ANT_HOLE_IRIS, interpolate};
use crate::{Drawable, GfxLayout, RectExt as _};

impl Drawable for &AntboxState {
    fn draw_on(self, g: &mut GfxLayout<'_>) {
        let gl = g.grid_layout;
        for (pt, rect) in gl.iter_pts_and_rects() {
            (self[pt], rect).draw_on(g);
        }
    }
}

impl Drawable for (Spot, Rect) {
    fn draw_on(self, g: &mut GfxLayout<'_>) {
        let (spot, rect) = self;
        spot.object().map(|obj| (obj, rect)).draw_on(g);
    }
}

impl Drawable for (Object, Rect) {
    fn draw_on(self, g: &mut GfxLayout<'_>) {
        use Object::*;

        let (obj, rect) = self;
        match obj {
            Food(x) => (x, rect).draw_on(g),
            Ant(x) => (x, rect).draw_on(g),
            AntHole(x) => (x, rect).draw_on(g),
        }
    }
}

impl Drawable for (SeedPod, Rect) {
    fn draw_on(self, g: &mut GfxLayout<'_>) {
        let (food, rect) = self;
        let crad = g.grid_layout.cell_radius * 0.9;

        let center = rect.center();
        let cellrotation = Angle::from(center.magnitude());
        let berrycolor = colors::food_neighbor_count(food.seeds);

        let c = food.seeds as f32;
        let theta = PI / c;

        let berryrad = {
            let magic_sauce = (theta * 0.71).sin();
            let seedf = if food.seeds == 1 {
                center.magnitude_squared().rem_euclid(1.0).powf(0.3)
            } else {
                1.0
            };
            crad * seedf * (magic_sauce / (1.1 + magic_sauce))
        };

        let spoke = TrigVec::new(PI / c, 0.8 * crad - berryrad);

        if food.ripe {
            g.draw_circle(center, crad, colors::FOOD_LIFE);
        }

        g.draw_circle(center, crad * 0.9, colors::SEEDPOD);
        for berry in 0..food.seeds {
            let mut bspoke = spoke.rotate(cellrotation + 2.0 * theta * berry as f32);

            if food.seeds == 1 {
                bspoke = bspoke.scale(center.magnitude().rem_euclid(1.0));
            }

            g.draw_circle(center + bspoke.into_vec2(), berryrad, berrycolor);
        }
    }
}

impl Drawable for (Ant, Rect) {
    fn draw_on(self, g: &mut GfxLayout<'_>) {
        // TODO: head, throax, abdomen, food pellet
        let (_, rect) = self;
        let rad = g.grid_layout.cell_radius * 0.5;
        g.draw_circle(rect.center(), rad, ANT);
    }
}

impl Drawable for (AntHole, Rect) {
    fn draw_on(self, g: &mut GfxLayout<'_>) {
        let (_, rect) = self;
        let center = rect.center();

        // Slightly too big to fit:
        let radbig = g.grid_layout.cell_radius * 1.3;
        let spoke = TrigVec::new(
            (center.x / center.y).asinh() * TAU,
            (center.x * center.y).rem_euclid(FRAC_1_SQRT_2).sin().abs() * radbig * 0.7,
        );

        let circles = 7;
        let invcirc = 1.0 / circles as f32;
        for i in 0..circles {
            let fdecay = 0.8f32.powi(i);
            let c = center + spoke.rotate(fdecay * TAU).scale(1.0 - fdecay);
            let rad = fdecay * radbig;
            let color = interpolate(ANT_HOLE_IRIS, ANT_HOLE_ENTRANCE, invcirc * i as f32);
            g.draw_circle(c, rad, color);
        }
    }
}
