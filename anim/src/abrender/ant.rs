use std::f32::consts::{FRAC_1_SQRT_2, TAU};

use antbox_gameboard::{Ant, AntHole};
use antbox_geom::{Distance, Polar, Rect, Transformable as _};
use antbox_render::{Backend, RenderWithArg};
use wyrand::WyRand;

use crate::abrender::AntboxRender;
use crate::colors::{ANT, ANT_HOLE_ENTRANCE, ANT_HOLE_IRIS, DIRT};

impl RenderWithArg<(Rect, &mut WyRand)> for AntboxRender<Ant> {
    fn render_with_arg<B: Backend>(self, rb: &mut B, (rect, wyr): (Rect, &mut WyRand)) {
        AntboxRender(self.0.pheromones_underneath()).render_with_arg(rb, (rect, wyr));

        // TODO: head, throax, abdomen, food pellet
        let rad: Distance = rect.inner_radius() * 0.5; // BUG: Why is this type-checking?
        rb.render(rect.center().with_radius(rad), ANT);
    }
}

impl RenderWithArg<(Rect, &mut WyRand)> for AntboxRender<AntHole> {
    fn render_with_arg<B: Backend>(self, rb: &mut B, (rect, wyr): (Rect, &mut WyRand)) {
        let center = rect.center();

        // Slightly too big to fit:
        let radbig = rect.inner_radius() * 1.7;
        let spoke = Polar::new(
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
                DIRT.interpolate(ANT_HOLE_IRIS, i as f32 / rimcircle as f32)
            } else {
                let j = i - rimcircle;
                let innercircles = circles - rimcircle;

                ANT_HOLE_IRIS.interpolate(ANT_HOLE_ENTRANCE, j as f32 / innercircles as f32)
            };
            rb.render(c.with_radius(rad), color);
        }
    }
}
