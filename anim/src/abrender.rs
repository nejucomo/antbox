mod ant;
mod seedpod;

use antbox_color::RED;
use antbox_float::{Norm, PowUnsigned as _};
use antbox_gameboard::{Pheromones, Spot};
use antbox_geom::{Distance, Point, Polar, Transformable as _};
use antbox_grid::Grid;
use antbox_render::{Backend, Colorable as _, RenderWithArg, Renderable};
use derive_more::From;
use rand::Rng as _;
use rand_distr::Distribution as _;
use wyrand::WyRand;

use crate::colors;
use crate::organic::OrganicScale;
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
        for (coords, rect) in layout.iter_coords_and_rects() {
            let mut wyr = wyrg[coords].clone();
            let mut view_layer = rb
                .transformation_layer()
                .scale_by_nnf(rect.inner_radius())
                .translate_by_point(rect.center());

            let orgscale = OrganicScale::default();
            let mut org_layer = view_layer
                .transformation_layer()
                .rotate_by_angle(wyr.random())
                .scale_by_nnf(orgscale.sample(&mut wyr))
                .translate(wyr.random::<Polar>().scale(Norm::HALF.pow_u32(2)));

            AntboxRender(self.0[coords]).render_with_arg(&mut org_layer, &mut wyr);
        }
    }
}

impl RenderWithArg<&mut WyRand> for AntboxRender<Spot> {
    fn render_with_arg<B: ?Sized + Backend>(self, rb: &mut B, wyr: &mut WyRand) {
        use Spot::*;

        match self.0 {
            Empty(x) => AntboxRender(x).render_with_arg(rb, wyr),
            Food(x) => AntboxRender(x).render_with_arg(rb, wyr),
            Ant(x) => AntboxRender(x).render_with_arg(rb, wyr),
            AntHole(x) => AntboxRender(x).render_with_arg(rb, wyr),
        }
    }
}

impl RenderWithArg<&mut WyRand> for AntboxRender<Pheromones> {
    fn render_with_arg<B: ?Sized + Backend>(self, rb: &mut B, wyr: &mut WyRand) {
        use colors::{ANT_HOLE_IRIS, FOOD_LIFE};

        const DECAY_FACTOR: Norm = Norm::fromp_f32(0.99);
        const HOME_COLOR_REDNESS: Norm = Norm::fromp_f32(0.4);

        let home_color = ANT_HOLE_IRIS.interpolate(RED, HOME_COLOR_REDNESS);
        let food_color = FOOD_LIFE;

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

            let spoke: Point = wyr.random::<Polar>().scale_by_f32(1.2).into();
            let radf = decay * decay.squeeze_up(Norm::HALF) * Norm::fromp_f32(0.8);
            let rad = Distance::fromp_f32(wyr.random_range(0.2..0.3)) * radf;

            rb.render(
                spoke.with_radius(rad).with_color(
                    color.with_alpha(decay.complement().squeeze_up(Norm::fromp_f32(0.2))),
                ),
            );
            decay = decay * DECAY_FACTOR;
        }
    }
}
