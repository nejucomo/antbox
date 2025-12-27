mod ant;
mod seedpod;

use antbox_gameboard::{Pheromones, Spot};
use antbox_geom::Grid;
use antbox_s2render::{
    RectExt as _, RenderScheduler, RenderWithArg, Renderable, Vec2Ext as _, WithColor as _,
};
use antbox_trig::TrigVec;
use derive_more::From;
use rand::Rng as _;
use speedy2d::color::Color;
use speedy2d::shape::Rect;
use wyrand::WyRand;

use crate::colors::{self, ColorExt as _};
use crate::layers::Layer;
use crate::{GridLayout, WyrGrid};

/// Convert into a [Renderable]
pub fn spots_into_renderable(
    spots: &Grid<Spot>,
    layout: GridLayout,
    wyrg: &WyrGrid,
) -> impl Renderable {
    AntBoxRender(spots).with_render_arg((layout, wyrg))
}

#[derive(Copy, Clone, Debug, From)]
struct AntBoxRender<T>(T);

impl RenderWithArg<(GridLayout, &WyrGrid)> for AntBoxRender<&Grid<Spot>> {
    fn schedule_with_arg(self, sch: &mut RenderScheduler, (layout, wyrg): (GridLayout, &WyrGrid)) {
        self.0.rwarg(sch, (layout, wyrg));
    }
}

trait RWArg<A> {
    fn rwarg(self, rs: &mut RenderScheduler, arg: A);
}

impl RWArg<(GridLayout, &WyrGrid)> for &Grid<Spot> {
    fn rwarg(self, rs: &mut RenderScheduler, (layout, wyrg): (GridLayout, &WyrGrid)) {
        for (pt, rect) in layout.iter_pts_and_rects() {
            let mut wyr = wyrg[pt].clone();
            self[pt].rwarg(rs, (rect, &mut wyr));
        }
    }
}

impl RWArg<(Rect, &mut WyRand)> for Spot {
    fn rwarg(self, rs: &mut RenderScheduler, arg: (Rect, &mut WyRand)) {
        use Spot::*;

        match self {
            Empty(x) => x.rwarg(rs, arg),
            Food(x) => x.rwarg(rs, arg),
            Ant(x) => x.rwarg(rs, arg),
            AntHole(x) => x.rwarg(rs, arg),
        }
    }
}

impl RWArg<(Rect, &mut WyRand)> for Pheromones {
    fn rwarg(self, rs: &mut RenderScheduler, (rect, wyr): (Rect, &mut WyRand)) {
        use colors::{ANT_HOLE_IRIS, FOOD_LIFE};

        const DECAY_FACTOR: f32 = 0.99;
        const HOME_COLOR_REDNESS: f32 = 0.4;

        let ls = Layer::Pheromones.layer_scheduler(rs);

        let home_color = ANT_HOLE_IRIS.interpolate(Color::RED, HOME_COLOR_REDNESS);
        let food_color = FOOD_LIFE;

        let center = rect.center();
        let crad = rect.cell_radius() * 0.9;

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

            ls.schedule(
                (center + spoke.into_vec2())
                    .with_radius(rad)
                    .with_color(color.with_alpha((1.0 - decay) * 0.8 + 0.2)),
            );
            decay *= DECAY_FACTOR;
        }
    }
}
