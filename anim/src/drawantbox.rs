use std::f32::consts::{FRAC_1_SQRT_2, PI, TAU};

use antbox_state::{
    Ant, AntHole, Object, Pheromone, Pheromones, SeedPod, Spot, State as AntboxState,
};
use antbox_trig::{Angle, TrigVec};
use speedy2d::Graphics2D;
use speedy2d::shape::Rect;

use crate::colors::{self, ANT, ANT_HOLE_ENTRANCE, ANT_HOLE_IRIS, ColorExt as _, interpolate};
use crate::{Drawable, GridLayout, RectExt as _};

impl Drawable<GridLayout> for &AntboxState {
    fn draw_on(self, gfx: &mut Graphics2D, layout: GridLayout) {
        for (pt, rect) in layout.iter_pts_and_rects() {
            self[pt].draw_on(gfx, rect);
        }
    }
}

impl Drawable<Rect> for Spot {
    fn draw_on(self, gfx: &mut Graphics2D, rect: Rect) {
        self.object().draw_on(gfx, rect.clone());
        self.pheromones().draw_on(gfx, rect);
    }
}

impl Drawable<Rect> for Object {
    fn draw_on(self, gfx: &mut Graphics2D, rect: Rect) {
        use Object::*;

        match self {
            Food(x) => x.draw_on(gfx, rect),
            Ant(x) => x.draw_on(gfx, rect),
            AntHole(x) => x.draw_on(gfx, rect),
        }
    }
}

impl Drawable<Rect> for SeedPod {
    fn draw_on(self, gfx: &mut Graphics2D, rect: Rect) {
        let crad = rect.cell_radius() * 0.9;

        let center = rect.center();
        let cellrotation = Angle::from(center.magnitude());
        let berrycolor = colors::food_neighbor_count(self.seeds);

        let c = self.seeds as f32;
        let theta = PI / c;

        let berryrad = {
            let magic_sauce = (theta * 0.71).sin();
            let seedf = if self.seeds == 1 {
                center.magnitude_squared().rem_euclid(1.0).powf(0.3)
            } else {
                1.0
            };
            crad * seedf * (magic_sauce / (1.1 + magic_sauce))
        };

        let spoke = TrigVec::new(PI / c, 0.8 * crad - berryrad);

        if self.ripe {
            gfx.draw_circle(center, crad, colors::FOOD_LIFE);
        }

        gfx.draw_circle(center, crad * 0.9, colors::SEEDPOD);
        for berry in 0..self.seeds {
            let mut bspoke = spoke.rotate(cellrotation + 2.0 * theta * berry as f32);

            if self.seeds == 1 {
                bspoke = bspoke.scale(center.magnitude().rem_euclid(1.0));
            }

            gfx.draw_circle(center + bspoke.into_vec2(), berryrad, berrycolor);
        }
    }
}

impl Drawable<Rect> for Ant {
    fn draw_on(self, gfx: &mut Graphics2D, rect: Rect) {
        // TODO: head, throax, abdomen, food pellet
        let rad = rect.cell_radius() * 0.5;
        gfx.draw_circle(rect.center(), rad, ANT);
    }
}

impl Drawable<Rect> for AntHole {
    fn draw_on(self, gfx: &mut Graphics2D, rect: Rect) {
        let center = rect.center();

        // Slightly too big to fit:
        let radbig = rect.cell_radius() * 1.3;
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
            gfx.draw_circle(c, rad, color);
        }
    }
}

impl Drawable<Rect> for Pheromones {
    fn draw_on(self, gfx: &mut Graphics2D, rect: Rect) {
        use Pheromone::{Food, Home};

        let crad = rect.cell_radius();
        let center = rect.center();
        let spoke = TrigVec::new(0f32, crad * 0.2);
        for ph in [Food, Home] {
            let mag = self.magnitude(ph);
            if mag > 0 {
                let magfactor = (mag.saturating_add(50) as f32 / u8::MAX as f32).sqrt();

                let rad = 0.8 * crad * magfactor;
                let (color, phang) = match ph {
                    Food => (colors::FOOD_LIFE, 0.),
                    Home => (colors::ANT_HOLE_IRIS, TAU / 3.),
                };

                let c = center + spoke.rotate(phang + magfactor * TAU);

                gfx.draw_circle(c, rad, color.with_alpha(magfactor));
            }
        }
    }
}
