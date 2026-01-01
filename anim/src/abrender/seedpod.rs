use std::f32::consts::PI;

use antbox_color::Color;
use antbox_float::Norm;
use antbox_gameboard::SeedPod;
use antbox_geom::{Angle, Distance, Point, Rect, Transformable as _};
use antbox_render::{Backend, Colorable as _, RenderWithArg};
use rand_distr::Distribution as _;
use wyrand::WyRand;

use crate::abrender::AntboxRender;
use crate::colors;
use crate::organic::OrganicScale;

// Components to split up rendering:
#[derive(Copy, Clone)]
struct DrawParams {
    org: OrganicScale,
    center: Point,
    podrad: Distance,
    spotrot: Angle,
    seedcolor: Color,
}

struct Pod(SeedPod);
struct SingletonSeed;
struct SeedCluster(SeedPod);

impl RenderWithArg<(Rect, &mut WyRand)> for AntboxRender<SeedPod> {
    fn render_with_arg<B: ?Sized + Backend>(self, rb: &mut B, (rect, wyr): (Rect, &mut WyRand)) {
        let seedpod = self.0;
        let org = OrganicScale::default();
        let center = rect.center();
        let dp = DrawParams {
            org,
            center,
            podrad: (rect.inner_radius() * 0.9 * org.sample(wyr))
                .try_into()
                .unwrap(),
            spotrot: Angle::from(center.distance_from_origin() * org.sample(wyr)),
            seedcolor: colors::food_neighbor_count(seedpod.seeds),
        };

        Pod(seedpod).render_with_arg(rb, (dp, wyr));

        if seedpod.seeds == 1 {
            SingletonSeed.render_with_arg(rb, (dp, wyr));
        } else {
            SeedCluster(seedpod).render_with_arg(rb, (dp, wyr));
        }
    }
}

impl RenderWithArg<(DrawParams, &mut WyRand)> for Pod {
    fn render_with_arg<B: ?Sized + Backend>(
        self,
        rb: &mut B,
        (dp, _wyr): (DrawParams, &mut WyRand),
    ) {
        let DrawParams { center, podrad, .. } = dp;
        let outer = center.with_radius(podrad);

        let Pod(SeedPod { seeds, ripe }) = self;

        let alpha_ripe = Norm::fromp_f32(if ripe { 0.75 } else { 0.25 });
        let alpha_seeds = Norm::fromp_f32(1.0 - 0.5 * ((seeds as f32 - 3.0).abs() / 5.0).powi(2));

        let life_color = colors::FOOD_LIFE.with_alpha(alpha_ripe * alpha_seeds);

        rb.render([
            outer.with_color(life_color),
            outer
                .scale_by_f32(0.9)
                .with_color(colors::SEEDPOD.interpolate(life_color, Norm::fromp_f32(0.1))),
        ]);
    }
}

impl RenderWithArg<(DrawParams, &mut WyRand)> for SingletonSeed {
    fn render_with_arg<B: ?Sized + Backend>(
        self,
        rb: &mut B,
        (dp, wyr): (DrawParams, &mut WyRand),
    ) {
        let DrawParams {
            org,
            center,
            podrad,
            spotrot,
            ..
        } = dp;

        let radf: f32 = 0.47 * org.sample(wyr);
        let distf = (org.sample(wyr) - radf).powi(2);

        for kernel in 0..2 {
            let offcenter = center
                + spotrot
                    .with_distance(podrad)
                    .scale_by_f32(distf)
                    .scale_by_f32(0.8f32.powi(kernel + 1));
            let circ = offcenter.with_radius(podrad).scale_by_f32(radf);
            let cwc = circ.scale_by_f32(0.6f32.powi(kernel));
            let color = colors::SEED.with_alpha(Norm::fromp_f32(1.0 - 0.9f32.powi(1 + kernel)));
            rb.render(cwc.with_color(color));
        }
    }
}

impl RenderWithArg<(DrawParams, &mut WyRand)> for SeedCluster {
    fn render_with_arg<B: ?Sized + Backend>(
        self,
        rb: &mut B,
        (dp, wyr): (DrawParams, &mut WyRand),
    ) {
        let DrawParams {
            org,
            center,
            podrad,
            spotrot,
            seedcolor,
        } = dp;

        let SeedCluster(SeedPod { seeds, ripe }) = self;
        assert_ne!(1, seeds);
        assert!(seeds <= 8);

        // The angle per seed spoke
        let theta = PI / seeds as f32;

        let radf = {
            let thetasin = theta.sin();
            org.sample(wyr) * thetasin / (1.0 + thetasin)
        };

        let spokef = spotrot.with_distance(Distance::fromp_f32(org.sample(wyr) - radf));

        // Draw the center flower hub:
        let (hub_circ, hub_color) = {
            let ripecenter = center + spokef.scale(podrad).scale_by_f32(0.1 * org.sample(wyr));

            let circ = ripecenter
                .with_radius(Distance::fromp_f32(org.sample(wyr) - radf))
                .scale_by_f32(0.6)
                .scale(podrad);

            let clr = {
                let (seedf, dirtf, alpha) = if ripe {
                    (0.55, 0.0, 0.95)
                } else {
                    (0.9, 0.3, 0.8)
                };
                let seedf = Norm::fromp_f32(seedf);
                let dirtf = Norm::fromp_f32(dirtf);
                let alpha = Norm::fromp_f32(alpha);

                colors::FOOD_LIFE
                    .interpolate(seedcolor, seedf)
                    .interpolate(colors::DIRT, dirtf)
                    .with_alpha(alpha)
            };

            (circ, clr)
        };

        rb.render(hub_circ.with_color(hub_color));

        for seed in 0..seeds {
            let bspokef = spokef.rotate(spotrot + 2.0 * org.sample(wyr) * theta * seed as f32);
            let bspoke = bspokef.scale(podrad);

            let seedcenter = center + bspoke;
            let seedcirc =
                seedcenter.with_radius(Distance::fromp_f32(podrad * radf * org.sample(wyr)));

            rb.render(seedcirc.with_color(seedcolor));
        }

        rb.render(
            hub_circ
                .scale_by_f32(0.7)
                .with_color(hub_color.with_alpha(Norm::fromp_f32(0.7))),
        );
    }
}
