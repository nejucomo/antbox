use std::f32::consts::PI;

use antbox_state::State;
use derive_more::{Deref, DerefMut, From};
use speedy2d::Graphics2D;
use speedy2d::dimen::Vec2;

use crate::angle::Angle;
use crate::colors;
use crate::trigvec::TrigVec;

/// Encapsulate all graphics rendering for a [State]
#[derive(Debug, From, Deref, DerefMut)]
pub(crate) struct StateGfx(State);

impl StateGfx {
    pub(crate) fn draw(&self, graphics: &mut Graphics2D, viewsize: Vec2) {
        let bounds = self.0.food.bounds();
        let w32 = bounds.width as f32;
        let h32 = bounds.height as f32;
        let cellsize = Vec2::new(viewsize.x / w32, viewsize.y / h32);

        graphics.clear_screen(colors::BACKGROUND);

        self.draw_food_neighbors(graphics, cellsize);
        // self.draw_food_life(graphics, cellsize);
    }

    fn draw_food_neighbors(&self, graphics: &mut Graphics2D, cellsize: Vec2) {
        let crad = cellsize.x.min(cellsize.y) / 2.0;

        for (pt, &cnt) in self.0.food.neighbor_counts().iter() {
            if cnt > 0 {
                let center = Vec2::new(
                    cellsize.x * (pt.x() as f32) + cellsize.x / 2.0,
                    cellsize.y * (pt.y() as f32) + cellsize.y / 2.0,
                );
                let cellrotation = Angle::from(center.magnitude());
                let berrycolor = colors::food_neighbor_count(cnt);

                let c = cnt as f32;
                let theta = PI / c;

                let berryrad = {
                    let magic_sauce = (theta * 0.71).sin();
                    crad * (magic_sauce / (1.1 + magic_sauce))
                };

                let spoke = TrigVec::new(PI / c, 0.8 * crad - berryrad);

                graphics.draw_circle(center, crad, colors::SEEDPOD);
                for berry in 0..cnt {
                    let bspoke = spoke.rotate(cellrotation + 2.0 * theta * berry as f32);
                    graphics.draw_circle(center + bspoke.into_vec2(), berryrad, berrycolor);
                }
            }
        }
    }

    #[allow(dead_code)]
    fn draw_food_life(&self, graphics: &mut Graphics2D, cellsize: Vec2) {
        let radius = cellsize.x.min(cellsize.y) / 2.0;

        for (pt, cell) in self.0.food.life().iter() {
            if cell.is_alive() {
                graphics.draw_circle(
                    (
                        cellsize.x * (pt.x() as f32) + cellsize.x / 2.0,
                        cellsize.y * (pt.y() as f32) + cellsize.y / 2.0,
                    ),
                    radius,
                    colors::FOOD,
                );
            }
        }
    }
}
