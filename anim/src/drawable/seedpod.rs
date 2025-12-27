use std::f32::consts::PI;

use antbox_gameboard::SeedPod;
use antbox_trig::Angle;
use speedy2d::Graphics2D;
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::shape::Rect;
use wyrand::WyRand;

use crate::{Drawable, RectExt as _, colors};

// Components to split up rendering:
#[derive(Copy, Clone)]
struct DrawParams {
    center: Vec2,
    podrad: f32,
    spotrot: Angle,
    seedcolor: Color,
}

struct Pod(bool);
struct SingletonSeed;
struct SeedCluster(u8);

impl Drawable<(Rect, &mut WyRand)> for SeedPod {
    fn draw_on(self, gfx: &mut Graphics2D, (rect, _wyr): (Rect, &mut WyRand)) {
        let center = rect.center();
        let dp = DrawParams {
            center,
            podrad: rect.cell_radius() * 0.9,
            spotrot: Angle::from(center.magnitude()),
            seedcolor: colors::food_neighbor_count(self.seeds),
        };

        Pod(self.ripe).draw_on(gfx, dp);

        if self.seeds == 1 {
            SingletonSeed.draw_on(gfx, dp);
        } else {
            SeedCluster(self.seeds).draw_on(gfx, dp);
        }
    }
}

impl Drawable<DrawParams> for Pod {
    fn draw_on(self, gfx: &mut Graphics2D, DrawParams { center, podrad, .. }: DrawParams) {
        if self.0 {
            gfx.draw_circle(center, podrad, colors::FOOD_LIFE);
        }

        gfx.draw_circle(center, podrad * 0.9, colors::SEEDPOD);
    }
}

impl Drawable<DrawParams> for SingletonSeed {
    fn draw_on(
        self,
        gfx: &mut Graphics2D,
        DrawParams {
            center,
            podrad,
            spotrot,
            ..
        }: DrawParams,
    ) {
        let radf: f32 = 0.47;
        let distf = (1.0 - radf).powi(2);

        let offcenter = center + spotrot.with_distance(podrad * distf);

        gfx.draw_circle(offcenter, podrad * radf, colors::SEEDPOD);
    }
}

impl Drawable<DrawParams> for SeedCluster {
    fn draw_on(
        self,
        gfx: &mut Graphics2D,
        DrawParams {
            center,
            podrad,
            spotrot,
            seedcolor,
        }: DrawParams,
    ) {
        let SeedCluster(seeds) = self;
        assert_ne!(1, seeds);
        assert!(seeds <= 8);

        // The angle per seed spoke
        let theta = PI / seeds as f32;

        let radf = {
            let thetasin = theta.sin();
            thetasin / (1.0 + thetasin)
        };

        let spokef = spotrot.with_distance(1.0 - radf);

        for seed in 0..seeds {
            let bspokef = spokef.rotate(spotrot + 2.0 * theta * seed as f32);
            let bspoke = bspokef.scale(podrad);

            gfx.draw_circle(center + bspoke, radf * podrad, seedcolor);
        }
    }
}
