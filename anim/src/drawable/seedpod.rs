use std::f32::consts::PI;

use antbox_gameboard::SeedPod;
use antbox_trig::Angle;
use rand_distr::Distribution as _;
use speedy2d::Graphics2D;
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::shape::Rect;
use wyrand::WyRand;

use crate::organic::OrganicScale;
use crate::{Drawable, RectExt as _, colors};

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

impl Drawable<(Rect, &mut WyRand)> for SeedPod {
    fn draw_on(self, gfx: &mut Graphics2D, (rect, wyr): (Rect, &mut WyRand)) {
        let org = OrganicScale::default();
        let center = rect.center();
        let dp = DrawParams {
            org,
            center,
            podrad: rect.cell_radius() * 0.9 * org.sample(wyr),
            spotrot: Angle::from(center.magnitude() * org.sample(wyr)),
            seedcolor: colors::food_neighbor_count(self.seeds),
        };

        Pod(self.ripe).draw_on(gfx, (dp, wyr));

        if self.seeds == 1 {
            SingletonSeed.draw_on(gfx, (dp, wyr));
        } else {
            SeedCluster(self.seeds).draw_on(gfx, (dp, wyr));
        }
    }
}

impl Drawable<(DrawParams, &mut WyRand)> for Pod {
    fn draw_on(self, gfx: &mut Graphics2D, (dp, _wyr): (DrawParams, &mut WyRand)) {
        let DrawParams { center, podrad, .. } = dp;

        if self.0 {
            gfx.draw_circle(center, podrad, colors::FOOD_LIFE);
        }

        gfx.draw_circle(center, podrad * 0.9, colors::SEEDPOD);
    }
}

impl Drawable<(DrawParams, &mut WyRand)> for SingletonSeed {
    fn draw_on(self, gfx: &mut Graphics2D, (dp, wyr): (DrawParams, &mut WyRand)) {
        let DrawParams {
            org,
            center,
            podrad,
            spotrot,
            ..
        } = dp;

        let radf: f32 = 0.47 * org.sample(wyr);
        let distf = (org.sample(wyr) - radf).powi(2);

        let offcenter = center + spotrot.with_distance(podrad * distf);

        gfx.draw_circle(offcenter, podrad * radf, colors::SEEDPOD);
    }
}

impl Drawable<(DrawParams, &mut WyRand)> for SeedCluster {
    fn draw_on(self, gfx: &mut Graphics2D, (dp, wyr): (DrawParams, &mut WyRand)) {
        let DrawParams {
            org,
            center,
            podrad,
            spotrot,
            seedcolor,
        } = dp;

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

            gfx.draw_circle(center + bspoke, radf * podrad * org.sample(wyr), seedcolor);
        }
    }
}
