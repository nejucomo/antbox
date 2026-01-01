mod ant;
mod seedpod;

use antbox_color::RED;
use antbox_gameboard::{Pheromones, Spot};
use antbox_geom::{Polar, Rect, Transformable as _};
use antbox_grid::Grid;
use antbox_render::{Backend, RenderWithArg, Renderable};
use derive_more::From;
use rand::Rng as _;
use wyrand::WyRand;

use crate::colors;
use crate::{GridLayout, WyrGrid};

/// Convert into a [Renderable]
pub fn spots_into_renderable(
    spots: &Grid<Spot>,
    layout: GridLayout,
    wyrg: &WyrGrid,
) -> impl Renderable {
    AntboxRender(spots).with_render_arg((layout, wyrg))
}

#[derive(Copy, Clone, Debug, From)]
struct AntboxRender<T>(T);

impl RenderWithArg<(GridLayout, &WyrGrid)> for AntboxRender<&Grid<Spot>> {
    fn render_with_arg<B: Backend>(self, rb: &mut B, (layout, wyrg): (GridLayout, &WyrGrid)) {
        for (pt, rect) in layout.iter_pts_and_rects() {
            let mut wyr = wyrg[pt].clone();
            AntboxRender(self.0[pt]).render_with_arg(rb, (rect, &mut wyr));
        }
    }
}

impl RenderWithArg<(Rect, &mut WyRand)> for AntboxRender<Spot> {
    fn render_with_arg<B: Backend>(self, rb: &mut B, arg: (Rect, &mut WyRand)) {
        use Spot::*;

        match self.0 {
            Empty(x) => AntboxRender(rb).render_with_arg(arg),
            Food(x) => AntboxRender(rb).render_with_arg(arg),
            Ant(x) => AntboxRender(rb).render_with_arg(arg),
            AntHole(x) => AntboxRender(rb).render_with_arg(arg),
        }
    }
}

impl RenderWithArg<(Rect, &mut WyRand)> for AntboxRender<Pheromones> {
    fn render_with_arg<B: Backend>(self, rb: &mut B, (rect, wyr): (Rect, &mut WyRand)) {
        use colors::{ANT_HOLE_IRIS, FOOD_LIFE};

        const DECAY_FACTOR: f32 = 0.99;
        const HOME_COLOR_REDNESS: f32 = 0.4;

        let home_color = ANT_HOLE_IRIS.interpolate(RED, HOME_COLOR_REDNESS);
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

            let spoke = wyr.random::<Polar>().scale(crad);
            let rad = wyr.random_range(0.2..0.3) * crad * decay * 0.4 * (1.0 + decay);

            rb.render(
                (center + spoke).with_radius(rad),
                color.with_alpha((1.0 - decay) * 0.8 + 0.2),
            );
            decay *= DECAY_FACTOR;
        }
    }
}
