mod ant;
mod seedpod;

use antbox_gameboard::{Pheromones, Spot};
use antbox_geom::{Polar, Rect, Transformable as _};
use antbox_grid::Grid;
use antbox_render::{Backend, RenderWithArg, Renderable};
use derive_more::From;
use rand::Rng as _;
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
    fn render_with_arg<B: Backend>(self, rb: &mut B, (layout, wyrg): (GridLayout, &WyrGrid)) {
        for (pt, rect) in layout.iter_pts_and_rects() {
            let mut wyr = wyrg[pt].clone();
            AntBoxRender(self.0[pt]).render_with_arg(rb, (rect, &mut wyr));
        }
    }
}

impl RenderWithArg<(Rect, &mut WyRand)> for AntBoxRender<Spot> {
    fn render_with_arg<B: Backend>(self, rb: &mut B, arg: (Rect, &mut WyRand)) {
        use Spot::*;

        match self.0 {
            Empty(x) => AntBoxRender(rb).render_with_arg(arg),
            Food(x) => AntBoxRender(rb).render_with_arg(arg),
            Ant(x) => AntBoxRender(rb).render_with_arg(arg),
            AntHole(x) => AntBoxRender(rb).render_with_arg(arg),
        }
    }
}

impl RenderWithArg<(Rect, &mut WyRand)> for AntBoxRender<Pheromones> {
    fn render_with_arg<B: Backend>(self, rb: &mut B, (rect, wyr): (Rect, &mut WyRand)) {
        use colors::{ANT_HOLE_IRIS, FOOD_LIFE};

        const DECAY_FACTOR: f32 = 0.99;
        const HOME_COLOR_REDNESS: f32 = 0.4;

        let ls = Layer::Pheromones.scheduler(rb);

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

            let spoke = wyr.random::<Polar>().scale(crad);
            let rad = wyr.random_range(0.2..0.3) * crad * decay * 0.4 * (1.0 + decay);

            ls.schedule(
                (center + spoke)
                    .with_radius(rad)
                    .with_color(color.with_alpha((1.0 - decay) * 0.8 + 0.2)),
            );
            decay *= DECAY_FACTOR;
        }
    }
}
