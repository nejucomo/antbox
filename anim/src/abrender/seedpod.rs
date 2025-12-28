use std::f32::consts::PI;

use antbox_gameboard::SeedPod;
use antbox_s2render::{RectExt as _, RenderCycle, Vec2Ext as _, WithColor};
use antbox_trig::Angle;
use rand_distr::Distribution as _;
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::shape::Rect;
use wyrand::WyRand;

use crate::abrender::RWArg;
use crate::colors::{self, ColorExt as _};
use crate::layers::Layer::Plants;
use crate::organic::OrganicScale;

// Components to split up rendering:
#[derive(Copy, Clone)]
struct DrawParams {
    org: OrganicScale,
    center: Vec2,
    podrad: f32,
    spotrot: Angle,
    seedcolor: Color,
}

struct Pod(SeedPod);
struct SingletonSeed;
struct SeedCluster(SeedPod);

impl RWArg<(Rect, &mut WyRand)> for SeedPod {
    fn rwarg(self, cycle: &mut RenderCycle, (rect, wyr): (Rect, &mut WyRand)) {
        let org = OrganicScale::default();
        let center = rect.center();
        let dp = DrawParams {
            org,
            center,
            podrad: rect.cell_radius() * 0.9 * org.sample(wyr),
            spotrot: Angle::from(center.magnitude() * org.sample(wyr)),
            seedcolor: colors::food_neighbor_count(self.seeds),
        };

        Pod(self).rwarg(cycle, (dp, wyr));

        if self.seeds == 1 {
            SingletonSeed.rwarg(cycle, (dp, wyr));
        } else {
            SeedCluster(self).rwarg(cycle, (dp, wyr));
        }
    }
}

impl RWArg<(DrawParams, &mut WyRand)> for Pod {
    fn rwarg(self, cycle: &mut RenderCycle, (dp, _wyr): (DrawParams, &mut WyRand)) {
        let DrawParams { center, podrad, .. } = dp;
        let ls = Plants.scheduler(cycle);
        let outer = center.with_radius(podrad);

        let Pod(SeedPod { seeds, ripe }) = self;

        let alpha_ripe = if ripe { 0.75 } else { 0.25 };
        let alpha_seeds = 1.0 - 0.5 * ((seeds as f32 - 3.0).abs() / 5.0).powi(2);

        let life_color = colors::FOOD_LIFE.with_alpha(alpha_ripe * alpha_seeds);

        ls.schedule(outer.with_color(life_color));
        ls.schedule(
            outer
                .scale(0.9)
                .with_color(colors::SEEDPOD.interpolate(life_color, 0.1)),
        );
    }
}

impl RWArg<(DrawParams, &mut WyRand)> for SingletonSeed {
    fn rwarg(self, cycle: &mut RenderCycle, (dp, wyr): (DrawParams, &mut WyRand)) {
        let DrawParams {
            org,
            center,
            podrad,
            spotrot,
            ..
        } = dp;
        let ls = Plants.scheduler(cycle);

        let radf: f32 = 0.47 * org.sample(wyr);
        let distf = (org.sample(wyr) - radf).powi(2);

        for kernel in 0..2 {
            let offcenter = center
                + spotrot
                    .with_distance(podrad)
                    .scale(distf)
                    .scale(0.8f32.powi(kernel + 1));
            let circ = offcenter.with_radius(podrad).scale(radf);
            let cwc = circ
                .scale(0.6f32.powi(kernel))
                .with_color(colors::SEED.with_alpha(1.0 - 0.9f32.powi(1 + kernel)));
            ls.schedule(cwc);
        }
    }
}

impl RWArg<(DrawParams, &mut WyRand)> for SeedCluster {
    fn rwarg(self, cycle: &mut RenderCycle, (dp, wyr): (DrawParams, &mut WyRand)) {
        let DrawParams {
            org,
            center,
            podrad,
            spotrot,
            seedcolor,
        } = dp;
        let ls = Plants.scheduler(cycle);

        let SeedCluster(SeedPod { seeds, ripe }) = self;
        assert_ne!(1, seeds);
        assert!(seeds <= 8);

        // The angle per seed spoke
        let theta = PI / seeds as f32;

        let radf = {
            let thetasin = theta.sin();
            org.sample(wyr) * thetasin / (1.0 + thetasin)
        };

        let spokef = spotrot.with_distance(org.sample(wyr) - radf);

        // Draw the center flower hub:
        let (hub_circ, hub_color) = {
            let ripecenter = center + spokef.scale(podrad).scale(0.1 * org.sample(wyr));

            let circ = ripecenter
                .with_radius(org.sample(wyr) - radf)
                .scale(0.6)
                .scale(podrad);

            let clr = {
                let (seedf, dirtf, alpha) = if ripe {
                    (0.55, 0.0, 0.95)
                } else {
                    (0.9, 0.3, 0.8)
                };

                colors::FOOD_LIFE
                    .interpolate(seedcolor, seedf)
                    .interpolate(colors::DIRT, dirtf)
                    .with_alpha(alpha)
            };

            (circ, clr)
        };

        ls.schedule(hub_circ.with_color(hub_color));

        for seed in 0..seeds {
            let bspokef = spokef.rotate(spotrot + 2.0 * org.sample(wyr) * theta * seed as f32);
            let bspoke = bspokef.scale(podrad);

            ls.schedule(
                (center + bspoke)
                    .with_radius(radf * podrad * org.sample(wyr))
                    .with_color(seedcolor),
            );
        }

        ls.schedule(hub_circ.scale(0.7).with_color(hub_color.with_alpha(0.7)));
    }
}
