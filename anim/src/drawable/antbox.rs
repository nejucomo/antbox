use antbox_gameboard::{Pheromones, Spot};
use antbox_geom::Grid;
use antbox_trig::TrigVec;
use rand::Rng as _;
use speedy2d::Graphics2D;
use speedy2d::color::Color;
use speedy2d::shape::Rect;
use wyrand::WyRand;

use crate::colors::{self, ColorExt as _};
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
