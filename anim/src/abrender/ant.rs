use std::f32::consts::{FRAC_1_SQRT_2, TAU};

use antbox_gameboard::{Ant, AntHole};
use antbox_s2render::{RectExt, RenderCycle, Vec2Ext as _, WithColor as _};
use antbox_trig::TrigVec;
use speedy2d::shape::Rect;
use wyrand::WyRand;

use crate::abrender::RWArg;
use crate::colors::{ANT, ANT_HOLE_ENTRANCE, ANT_HOLE_IRIS, DIRT, interpolate};
use crate::layers::Layer;

impl RWArg<(Rect, &mut WyRand)> for Ant {
    fn rwarg(self, rs: &mut RenderCycle, (rect, wyr): (Rect, &mut WyRand)) {
        self.pheromones_underneath().rwarg(rs, (rect.clone(), wyr));

        // TODO: head, throax, abdomen, food pellet
        let rad = rect.cell_radius() * 0.5;
        Layer::Ants
            .scheduler(rs)
            .schedule(rect.center().with_radius(rad).with_color(ANT));
    }
}

impl RWArg<(Rect, &mut WyRand)> for AntHole {
    fn rwarg(self, rs: &mut RenderCycle, (rect, _wyr): (Rect, &mut WyRand)) {
        let ls = Layer::AntHole.scheduler(rs);

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
            ls.schedule(c.with_radius(rad).with_color(color));
        }
    }
}
