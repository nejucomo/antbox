use std::f32::consts::{FRAC_1_SQRT_2, TAU};

use antbox_float::{NNF, Norm, PowUnsigned as _};
use antbox_gameboard::{Ant, AntHole};
use antbox_geom::{Distance, Polar, Rect, Transformable as _};
use antbox_render::{Backend, Colorable as _, RenderWithArg};
use wyrand::WyRand;

use crate::abrender::AntboxRender;
use crate::colors::{ANT, ANT_HOLE_ENTRANCE, ANT_HOLE_IRIS, DIRT};

impl RenderWithArg<(Rect, &mut WyRand)> for AntboxRender<Ant> {
    fn render_with_arg<B: ?Sized + Backend>(self, rb: &mut B, (rect, wyr): (Rect, &mut WyRand)) {
        AntboxRender(self.0.pheromones_underneath()).render_with_arg(rb, (rect, wyr));

        // TODO: head, throax, abdomen, food pellet
        rb.render(rect.inner_circle().scale(Norm::HALF).with_color(ANT));
    }
}

impl RenderWithArg<(Rect, &mut WyRand)> for AntboxRender<AntHole> {
    fn render_with_arg<B: ?Sized + Backend>(self, rb: &mut B, (rect, _wyr): (Rect, &mut WyRand)) {
        let center = rect.center();

        // Slightly too big to fit:
        let radbig = rect.inner_radius() * NNF::fromp_f32(1.7);
        let spoke = Polar::new(
            (center.x / center.y).asinh() * TAU,
            Distance::fromp_f32(
                radbig * (center.x * center.y).rem_euclid(FRAC_1_SQRT_2).sin().abs() * 0.7,
            ),
        );

        let circles: usize = 7;
        let rimcircle: usize = 3;
        for i in 0..circles {
            let fdecay = Norm::fromp_f32(0.8f32).pow_u32(i.try_into().unwrap());
            let c = center
                + spoke
                    .rotate(f32::from(fdecay) * TAU)
                    .scale(fdecay.complement().pow_u32(3));
            let rad = fdecay * radbig;
            let color = if i <= rimcircle {
                DIRT.interpolate(ANT_HOLE_IRIS, Norm::fromp_ratio(i, rimcircle))
            } else {
                let j = i - rimcircle;
                let innercircles = circles - rimcircle;

                ANT_HOLE_IRIS.interpolate(ANT_HOLE_ENTRANCE, Norm::fromp_ratio(j, innercircles))
            };
            rb.render(c.with_radius(rad).with_color(color));
        }
    }
}
