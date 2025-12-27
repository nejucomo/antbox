use std::f32::consts::PI;

use antbox_gameboard::SeedPod;
use antbox_s2render::{RectExt as _, RenderScheduler, Vec2Ext as _, WithColor};
use antbox_trig::Angle;
use rand_distr::Distribution as _;
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::shape::Rect;
use wyrand::WyRand;

use crate::abrender::RWArg;
use crate::colors;
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

struct Pod(bool);
struct SingletonSeed;
struct SeedCluster(u8);

impl RWArg<(Rect, &mut WyRand)> for SeedPod {
    fn rwarg(self, rs: &mut RenderScheduler, (rect, wyr): (Rect, &mut WyRand)) {
        let org = OrganicScale::default();
        let center = rect.center();
        let dp = DrawParams {
            org,
            center,
            podrad: rect.cell_radius() * 0.9 * org.sample(wyr),
            spotrot: Angle::from(center.magnitude() * org.sample(wyr)),
            seedcolor: colors::food_neighbor_count(self.seeds),
        };

        Pod(self.ripe).rwarg(rs, (dp, wyr));

        if self.seeds == 1 {
            SingletonSeed.rwarg(rs, (dp, wyr));
        } else {
            SeedCluster(self.seeds).rwarg(rs, (dp, wyr));
        }
    }
}

impl RWArg<(DrawParams, &mut WyRand)> for Pod {
    fn rwarg(self, rs: &mut RenderScheduler, (dp, _wyr): (DrawParams, &mut WyRand)) {
        let DrawParams { center, podrad, .. } = dp;
        let ls = Plants.layer_scheduler(rs);

        let outer = center.with_radius(podrad);
        if self.0 {
            ls.schedule(outer.with_color(colors::FOOD_LIFE));
        }

        ls.schedule(outer.scale(0.9).with_color(colors::SEEDPOD));
    }
}

impl RWArg<(DrawParams, &mut WyRand)> for SingletonSeed {
    fn rwarg(self, rs: &mut RenderScheduler, (dp, wyr): (DrawParams, &mut WyRand)) {
        let DrawParams {
            org,
            center,
            podrad,
            spotrot,
            ..
        } = dp;
        let ls = Plants.layer_scheduler(rs);

        let radf: f32 = 0.47 * org.sample(wyr);
        let distf = (org.sample(wyr) - radf).powi(2);

        let offcenter = center + spotrot.with_distance(podrad).scale(distf);

        ls.schedule(
            offcenter
                .with_radius(podrad)
                .scale(radf)
                .with_color(colors::SEEDPOD),
        );
    }
}

impl RWArg<(DrawParams, &mut WyRand)> for SeedCluster {
    fn rwarg(self, rs: &mut RenderScheduler, (dp, wyr): (DrawParams, &mut WyRand)) {
        let DrawParams {
            org,
            center,
            podrad,
            spotrot,
            seedcolor,
        } = dp;
        let ls = Plants.layer_scheduler(rs);

        let SeedCluster(seeds) = self;
        assert_ne!(1, seeds);
        assert!(seeds <= 8);

        // The angle per seed spoke
        let theta = PI / seeds as f32;

        let radf = {
            let thetasin = theta.sin();
            org.sample(wyr) * thetasin / (1.0 + thetasin)
        };

        let spokef = spotrot.with_distance(org.sample(wyr) - radf);

        for seed in 0..seeds {
            let bspokef = spokef.rotate(spotrot + 2.0 * org.sample(wyr) * theta * seed as f32);
            let bspoke = bspokef.scale(podrad);

            ls.schedule(
                (center + bspoke)
                    .with_radius(radf * podrad * org.sample(wyr))
                    .with_color(seedcolor),
            );
        }
    }
}
