use std::f32::consts::TAU;

use antbox_float::{NNF, Norm, PowUnsigned as _};
use antbox_gameboard::{Ant, AntHole};
use antbox_geom::{Circle, Point, Polar, Transformable as _};
use antbox_render::{Backend, Colorable as _, RenderWithArg};
use rand::Rng as _;
use wyrand::WyRand;

use crate::abrender::AntboxRender;
use crate::colors::{ANT, ANT_HOLE_ENTRANCE, ANT_HOLE_IRIS, DIRT};

impl RenderWithArg<&mut WyRand> for AntboxRender<Ant> {
    fn render_with_arg<B: ?Sized + Backend>(self, rb: &mut B, wyr: &mut WyRand) {
        AntboxRender(self.0.pheromones_underneath()).render_with_arg(rb, wyr);

        // TODO: head, throax, abdomen, food pellet
        rb.render(Circle::UNIT.scale(Norm::HALF).with_color(ANT));
    }
}

impl RenderWithArg<&mut WyRand> for AntboxRender<AntHole> {
    fn render_with_arg<B: ?Sized + Backend>(self, rb: &mut B, wyr: &mut WyRand) {
        // Slightly too big to fit:
        let radbig = NNF::fromp_f32(1.7);
        let spoke = wyr.random::<Polar>().scale(radbig * Norm::HALF);

        let circles: usize = 7;
        let rimcircle: usize = 3;
        for i in 0..circles {
            let fdecay = Norm::fromp_f32(0.8f32).pow_u32(i.try_into().unwrap());
            let c = spoke
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
            rb.render(Point::from(c).with_radius(rad).with_color(color));
        }
    }
}
