mod ant;
mod seedpod;

use antbox_color::RED;
use antbox_float::Norm;
use antbox_gameboard::{Pheromones, Spot};
use antbox_geom::{Distance, Polar, Rect, Transformable as _};
use antbox_grid::Grid;
use antbox_render::{Backend, Colorable as _, RenderWithArg, Renderable};
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
    fn render_with_arg<B: ?Sized + Backend>(
        self,
        rb: &mut B,
        (layout, wyrg): (GridLayout, &WyrGrid),
    ) {
        for (pt, rect) in layout.iter_pts_and_rects() {
            let mut wyr = wyrg[pt].clone();
            AntboxRender(self.0[pt]).render_with_arg(rb, (rect, &mut wyr));
        }
    }
}

impl RenderWithArg<(Rect, &mut WyRand)> for AntboxRender<Spot> {
    fn render_with_arg<B: ?Sized + Backend>(self, rb: &mut B, arg: (Rect, &mut WyRand)) {
        use Spot::*;

        match self.0 {
            Empty(x) => AntboxRender(x).render_with_arg(rb, arg),
            Food(x) => AntboxRender(x).render_with_arg(rb, arg),
            Ant(x) => AntboxRender(x).render_with_arg(rb, arg),
            AntHole(x) => AntboxRender(x).render_with_arg(rb, arg),
        }
    }
}

impl RenderWithArg<(Rect, &mut WyRand)> for AntboxRender<Pheromones> {
    fn render_with_arg<B: ?Sized + Backend>(self, rb: &mut B, (rect, wyr): (Rect, &mut WyRand)) {
        use colors::{ANT_HOLE_IRIS, FOOD_LIFE};

        const DECAY_FACTOR: Norm = Norm::fromp_f32(0.99);
        const HOME_COLOR_REDNESS: Norm = Norm::fromp_f32(0.4);

        let home_color = ANT_HOLE_IRIS.interpolate(RED, HOME_COLOR_REDNESS);
        let food_color = FOOD_LIFE;

        let center = rect.center();
        let crad = Distance::try_from(rect.inner_radius() * 0.9).unwrap();

        let mut decay = Norm::ONE;
        let mut ph = self.0;
        while !ph.is_empty() {
            let color = if wyr.random_ratio(ph.food as u32, ph.food as u32 + ph.home as u32) {
                ph.food -= 1;
                food_color
            } else {
                ph.home -= 1;
                home_color
            };

            let spoke = wyr.random::<Polar>().scale(crad);
            let radf = decay * decay.squeeze_up(Norm::HALF) * Norm::fromp_f32(0.8);
            let rad = Distance::fromp_f32(crad * radf * wyr.random_range(0.2..0.3));

            rb.render(
                (center + spoke).with_radius(rad).with_color(
                    color.with_alpha(decay.complement().squeeze_up(Norm::fromp_f32(0.2))),
                ),
            );
            decay = decay * DECAY_FACTOR;
        }
    }
}
