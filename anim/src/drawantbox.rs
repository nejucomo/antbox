use std::f32::consts::{FRAC_1_SQRT_2, PI, TAU};

use antbox_geom::Grid;
use antbox_state::{Ant, AntHole, Pheromones, SeedPod, Spot};
use antbox_trig::{Angle, TrigVec};
use rand::Rng as _;
use speedy2d::Graphics2D;
use speedy2d::color::Color;
use speedy2d::shape::Rect;
use wyrand::WyRand;

use crate::colors::{
    self, ANT, ANT_HOLE_ENTRANCE, ANT_HOLE_IRIS, ColorExt as _, DIRT, interpolate,
};
use crate::{Drawable, GridLayout, RectExt as _, WyrGrid};

impl Drawable<(GridLayout, &WyrGrid)> for &Grid<Spot> {
    fn draw_on(self, gfx: &mut Graphics2D, (layout, wyrgrid): (GridLayout, &WyrGrid)) {
        for (pt, rect) in layout.iter_pts_and_rects() {
            let mut wyr = wyrgrid[pt].clone();
            self[pt].draw_on(gfx, (rect, &mut wyr));
        }
    }
}

impl Drawable<(Rect, &mut WyRand)> for Spot {
    fn draw_on(self, gfx: &mut Graphics2D, params: (Rect, &mut WyRand)) {
        use Spot::*;

        match self {
            Empty(x) => x.draw_on(gfx, params),
            Food(x) => x.draw_on(gfx, params),
            Ant(x) => x.draw_on(gfx, params),
            AntHole(x) => x.draw_on(gfx, params),
        }
    }
}

impl Drawable<(Rect, &mut WyRand)> for SeedPod {
    fn draw_on(self, gfx: &mut Graphics2D, (rect, _wyr): (Rect, &mut WyRand)) {
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

impl Drawable<(Rect, &mut WyRand)> for Ant {
    fn draw_on(self, gfx: &mut Graphics2D, (rect, wyr): (Rect, &mut WyRand)) {
        self.pheromones_underneath()
            .draw_on(gfx, (rect.clone(), wyr));

        // TODO: head, throax, abdomen, food pellet
        let rad = rect.cell_radius() * 0.5;
        gfx.draw_circle(rect.center(), rad, ANT);
    }
}

impl Drawable<(Rect, &mut WyRand)> for AntHole {
    fn draw_on(self, gfx: &mut Graphics2D, (rect, _wyr): (Rect, &mut WyRand)) {
        let center = rect.center();

        // Slightly too big to fit:
        let radbig = rect.cell_radius() * 1.7;
        let spoke = TrigVec::new(
            (center.x / center.y).asinh() * TAU,
            (center.x * center.y).rem_euclid(FRAC_1_SQRT_2).sin().abs() * radbig * 0.7,
        );

        let circles = 7;
        let rimcircle = 3;
        for i in 0..circles {
            let fdecay = 0.8f32.powi(i);
            let c = center + spoke.rotate(fdecay * TAU).scale((1.0 - fdecay).powi(3));
            let rad = fdecay * radbig;
            let color = if i <= rimcircle {
                interpolate(DIRT, ANT_HOLE_IRIS, i as f32 / rimcircle as f32)
            } else {
                let j = i - rimcircle;
                let innercircles = circles - rimcircle;
                interpolate(
                    ANT_HOLE_IRIS,
                    ANT_HOLE_ENTRANCE,
                    j as f32 / innercircles as f32,
                )
            };
            gfx.draw_circle(c, rad, color);
        }
    }
}

impl Drawable<(Rect, &mut WyRand)> for Pheromones {
    fn draw_on(self, gfx: &mut Graphics2D, (rect, wyr): (Rect, &mut WyRand)) {
        use colors::{ANT_HOLE_IRIS, FOOD_LIFE};

        const DECAY_FACTOR: f32 = 0.99;
        const HOME_COLOR_REDNESS: f32 = 0.4;

        let home_color = ANT_HOLE_IRIS.interpolate(Color::RED, HOME_COLOR_REDNESS);
        let food_color = FOOD_LIFE;

        let center = rect.center();
        let crad = rect.cell_radius();

        let mut decay = 1.0;
        let mut ph = self;
        while !ph.is_empty() {
            let color = if wyr.random_ratio(ph.food as u32, ph.food as u32 + ph.home as u32) {
                ph.food -= 1;
                food_color
            } else {
                ph.home -= 1;
                home_color
            };

            let spoke = wyr.random::<TrigVec>().scale(crad);
            let rad = wyr.random_range(0.2..0.3) * crad * decay * 0.4 * (1.0 + decay);
            gfx.draw_circle(
                center + spoke.into_vec2(),
                rad,
                color.with_alpha((1.0 - decay) * 0.8 + 0.2),
            );
            decay *= DECAY_FACTOR;
        }
    }
}
