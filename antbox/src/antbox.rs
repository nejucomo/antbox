use antbox_geom::Bounds;
use cellauto::{GenGen, Generation};
use rand::Rng;
use rand::distr::Distribution;

use speedy2d::dimen::UVec2;

use crate::TickTimer;

pub struct AntBox<R>
where
    R: Rng + 'static,
{
    #[allow(dead_code)]
    rng: R,
    viewsize: UVec2,
    foodgrid: Generation,
    tt: TickTimer,
}

impl<R> AntBox<R>
where
    R: Rng + 'static,
{
    pub fn new(mut rng: R, cellprob: f64, viewsize: UVec2) -> Self
    where
        R: Rng,
    {
        let foodgrid = {
            let w = viewsize.x / crate::window::CELL_LENGTH;
            let h = viewsize.y / crate::window::CELL_LENGTH;
            let gg = GenGen::new(cellprob, Bounds::new(w, h));

            gg.sample(&mut rng)
        };

        let tt = TickTimer::default();

        AntBox {
            rng,
            viewsize,
            foodgrid,
            tt,
        }
    }

    pub fn food_grid(&self) -> &Generation {
        &self.foodgrid
    }
}

impl<R> std::fmt::Debug for AntBox<R>
where
    R: Rng + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AntBox")
            .field("rng", &"..")
            .field("viewsize", &self.viewsize)
            .field("foodgrid", &self.foodgrid)
            .field("tt", &self.tt)
            .finish()
    }
}
